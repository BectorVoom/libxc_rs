//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 725/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk725(t2079: f64, t7724: f64, t599: f64, t930: f64, t1181: f64, t2068: f64, t121: f64, t939: f64, t382: f64, t151: f64, t947: f64, t1004: f64, t1997: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7725 = t2079 * t7724;
    let t7727 = t599 * t930;
    let t7728 = t1181 * t7727;
    let t7729 = t2068 * t7728;
    let t7731 = t939 * t121;
    let t7732 = t7731 * t382;
    let t7733 = t151 * t7732;
    let t7734 = t7733 * t947;
    let t7736 = t1004 * t1997;
    (t7725, t7727, t7728, t7729, t7731, t7732, t7734, t7736)
}
