//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1103/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1103<F: Float>(t4167: F, t828: F, t2275: F, t4166: F, t10534: F, t10549: F, t6530: F, t6749: F, t8676: F, t8808: F, t10552: F, t10720: F, t10803: F, t10806: F, t10810: F, t10817: F, t10820: F, t10823: F, t2251: F, t2273: F, t2312: F, t271: F, t3371: F, t3390: F, t6667: F, t6683: F, t8821: F, t8862: F) -> (F, F, F, F, F) {
    let t10826 = t4167 * t828;
    let t10829 = t4166 * t2275;
    let t10830 = t10829 * t828;
    let t10838 = -t6749 + F::cast_from(0.22831111111111111111e-1_f64) * t6530 + F::cast_from(0.45662222222222222221e-1_f64) * t8676 - t8808 - F::cast_from(0.17123333333333333333e-1_f64) * t10534 + F::cast_from(0.5137e-1_f64) * t10549;
    let t10841 = F::cast_from(0.17315859105681463759e2_f64) * t2312 * t10803 + F::cast_from(0.34631718211362927518e2_f64) * t2312 * t10806 + F::cast_from(0.10254018858216406658e4_f64) * t6667 * t10810 - F::cast_from(4.0_f64) * t8821 * t3371 + F::cast_from(0.64327917994770140268e2_f64) * t8862 * t3390 + F::cast_from(6.0_f64) * t2273 * t10817 - F::cast_from(4.0_f64) * t2251 * t10820 - F::cast_from(0.19298375398431042081e3_f64) * t6683 * t10823 - F::cast_from(2.0_f64) * t2251 * t10826 + F::cast_from(0.32163958997385070134e2_f64) * t2273 * t10830 - F::cast_from(0.19751673498613801407e-1_f64) * t10552 - F::cast_from(0.310907e-1_f64) * t10838 * t271 + t10720;
    (t10826, t10829, t10830, t10838, t10841)
}
