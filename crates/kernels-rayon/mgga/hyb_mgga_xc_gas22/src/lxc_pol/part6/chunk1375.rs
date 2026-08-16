//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1375/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1375(t10948: f64, t238: f64, t801: f64, t242: f64, t2466: f64, t4234: f64, t10911: f64, t940: f64, t29786: f64, t343: f64, t2213: f64, t4261: f64) -> (f64, f64, f64, f64, f64) {
    let t29884 = t238 * t801 * t10948;
    let t29888 = t238 * t242 * t2466 * t4234;
    let t29892 = t238 * t242 * t940 * t10911;
    let t29896 = t238 * t242 * t343 * t29786;
    let t29905 = t238 * t2213 * t4261;
    (t29884, t29888, t29892, t29896, t29905)
}
