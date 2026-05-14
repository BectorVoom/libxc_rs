//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1052/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1052<F: Float>(t10552: F, t10720: F, t10803: F, t10806: F, t10810: F, t10817: F, t10820: F, t10823: F, t10826: F, t10830: F, t10838: F, t2251: F, t2273: F, t2312: F, t271: F, t3371: F, t3390: F, t6667: F, t6683: F, t8821: F, t8862: F) -> (F,) {
    let t10841 = 0.17315859105681463759e2 * t2312 * t10803 + 0.34631718211362927518e2 * t2312 * t10806 + 0.10254018858216406658e4 * t6667 * t10810 - 4.0 * t8821 * t3371 + 0.64327917994770140268e2 * t8862 * t3390 + 6.0 * t2273 * t10817 - 4.0 * t2251 * t10820 - 0.19298375398431042081e3 * t6683 * t10823 - 2.0 * t2251 * t10826 + 0.32163958997385070134e2 * t2273 * t10830 - 0.19751673498613801407e-1 * t10552 - 0.310907e-1 * t10838 * t271 + t10720;
    (t10841,)
}
