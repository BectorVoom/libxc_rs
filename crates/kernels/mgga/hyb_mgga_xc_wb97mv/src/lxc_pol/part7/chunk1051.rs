//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1051/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1051<F: Float>(t1312: F, t3289: F, t26: F, t10: F, t10621: F, t18: F, t1285: F, t2967: F, t1313: F, t4109: F, t763: F, t1234: F, t3174: F, t3296: F, t4072: F, t549: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10853 = t3289 * t1312;
    let t10854 = t26 * t10853;
    let t10862 = t10621 * t10 * t18;
    let t10869 = t2967 * t1285;
    let t10874 = t2967 * t1313;
    let t10877 = t763 * t4109;
    let t10878 = t26 * t10877;
    let t10881 = t1234 * t3174;
    let t10883 = t1234 * t3296;
    let t10885 = t549 * t4072;
    (t10853, t10854, t10862, t10869, t10874, t10877, t10878, t10881, t10883, t10885)
}
