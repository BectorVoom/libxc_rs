//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1422/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1422(t11478: f64, t9478: f64, t11469: f64, t1828: f64, t3748: f64, t11474: f64, t1834: f64, t313: f64, t3951: f64, t3957: f64, t11376: f64, t22723: f64, t412: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30760 = t11478 * t9478;
    let t30763 = t11469 * t1828;
    let t30764 = t3748 * t30763;
    let t30767 = t11474 * t9478;
    let t30771 = t3951 * t313 * t1834;
    let t30772 = t3748 * t30771;
    let t30776 = t3957 * t313 * t1834;
    let t30777 = t11376 * t30776;
    let t30781 = t22723 * t412 * t30776;
    (t30760, t30763, t30764, t30767, t30771, t30772, t30776, t30777, t30781)
}
