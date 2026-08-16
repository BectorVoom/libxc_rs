//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 858/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk858(t2628: f64, t836: f64, t812: f64, t242: f64, t9972: f64, t2638: f64, t4166: f64, t2629: f64, t820: f64, t9645: f64, t2696: f64, t1516: f64, t9601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13257 = t2628 * t836;
    let t13258 = t812 * t13257;
    let t13261 = t9972 * t242;
    let t13262 = t812 * t13261;
    let t13278 = t4166 * t2638;
    let t13283 = t4166 * t2629;
    let t13350 = t9645 * t820;
    let t13360 = t4166 * t2696;
    let t13368 = t9601 * t1516;
    (t13258, t13262, t13278, t13283, t13350, t13360, t13368)
}
