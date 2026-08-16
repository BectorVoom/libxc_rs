//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1103/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1103(t4167: f64, t828: f64, t2275: f64, t4166: f64, t10534: f64, t10549: f64, t6530: f64, t6749: f64, t8676: f64, t8808: f64, t10552: f64, t10720: f64, t10803: f64, t10806: f64, t10810: f64, t10817: f64, t10820: f64, t10823: f64, t2251: f64, t2273: f64, t2312: f64, t271: f64, t3371: f64, t3390: f64, t6667: f64, t6683: f64, t8821: f64, t8862: f64) -> (f64, f64, f64, f64, f64) {
    let t10826 = t4167 * t828;
    let t10829 = t4166 * t2275;
    let t10830 = t10829 * t828;
    let t10838 = -t6749 + 0.22831111111111111111e-1_f64 * t6530 + 0.45662222222222222221e-1_f64 * t8676 - t8808 - 0.17123333333333333333e-1_f64 * t10534 + 0.5137e-1_f64 * t10549;
    let t10841 = 0.17315859105681463759e2_f64 * t2312 * t10803 + 0.34631718211362927518e2_f64 * t2312 * t10806 + 0.10254018858216406658e4_f64 * t6667 * t10810 - 4.0_f64 * t8821 * t3371 + 0.64327917994770140268e2_f64 * t8862 * t3390 + 6.0_f64 * t2273 * t10817 - 4.0_f64 * t2251 * t10820 - 0.19298375398431042081e3_f64 * t6683 * t10823 - 2.0_f64 * t2251 * t10826 + 0.32163958997385070134e2_f64 * t2273 * t10830 - 0.19751673498613801407e-1_f64 * t10552 - 0.310907e-1_f64 * t10838 * t271 + t10720;
    (t10826, t10829, t10830, t10838, t10841)
}
