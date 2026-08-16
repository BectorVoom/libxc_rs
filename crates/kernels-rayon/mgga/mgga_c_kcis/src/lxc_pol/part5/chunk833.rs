//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 833/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk833(t381: f64, t6708: f64, t389: f64, t1813: f64, t5172: f64, t1809: f64, t1817: f64, t388: f64, t6486: f64, t387: f64, t3442: f64, t3438: f64, t6491: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6709 = t6708 * t381;
    let t6710 = t6709 * t389;
    let t6712 = t5172 * t1813;
    let t6714 = t1809 * t1817;
    let t6716 = t388 * t6486;
    let t6717 = t387 * t6716;
    let t6718 = t3442 * t6717;
    let t6720 = t3438 * t6491;
    (t6709, t6710, t6712, t6714, t6716, t6717, t6718, t6720)
}
