//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1070/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1070(t3131: f64, t4649: f64, t4593: f64, t4582: f64, t16558: f64, t998: f64, t974: f64, t13835: f64, t4531: f64, t13769: f64, t13839: f64, t1539: f64, t6733: f64) -> (f64, f64, f64, f64, f64) {
    let t17732 = t3131 * t4649;
    let t17733 = t4593 * t17732;
    let t17734 = t4582 * t17733;
    let t17737 = t998 * t16558;
    let t17738 = t974 * t17737;
    let t17742 = t4531 * t13835;
    let t17745 = t13769 * t13839;
    let t17748 = t6733 * t1539;
    (t17734, t17738, t17742, t17745, t17748)
}
