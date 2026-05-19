//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 853/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk853<F: Float>(t2274: F, t270: F, t2289: F, t835: F, t2250: F, t816: F, t2282: F, t839: F, t2306: F, t2314: F, t6527: F, t260: F) -> (F, F, F, F, F, F, F) {
    let t6712 = F::new(1.0) / t2274 / t270;
    let t6716 = t835 * t2289;
    let t6722 = t816 * t2250;
    let t6729 = t2282 * t839;
    let t6737 = t2306 * t2314;
    let t6749 = F::cast_from(0.53272592592592592592e-1_f64) * t6527;
    let t6759 = t260 * t2282;
    (t6712, t6716, t6722, t6729, t6737, t6749, t6759)
}
