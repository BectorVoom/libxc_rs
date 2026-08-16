//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1823/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1823(t2031: f64, t83718: f64, t2240: f64, t240: f64, t33: f64, t6492: f64, t2244: f64, t63: f64, t23993: f64, t6495: f64, t1860: f64, t22489: f64, t7031: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t84237 = t2031 * t83718;
    let t84241 = t2240 * t33 * t240;
    let t84242 = t84241 * t6492;
    let t84245 = t2240 * t2244 * t63;
    let t84248 = t6495 * t23993;
    let t84270 = t1860 * t7031 * t22489;
    (t84237, t84241, t84242, t84245, t84248, t84270)
}
