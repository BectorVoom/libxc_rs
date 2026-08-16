//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1059/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1059(t33953: f64, t4241: f64, t13364: f64, t34833: f64, t13299: f64, t2001: f64, t4344: f64, t4349: f64, t7741: f64, t2290: f64, t7630: f64, t1549: f64, t30540: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34834 = t33953 * t4241;
    let t34836 = t34833 * t13364 * t34834;
    let t34839 = t34833 * t13299 * t34834;
    let t34841 = t2001 * t4344;
    let t34844 = t7741 * t4349;
    let t34849 = t7630 * t2290;
    let t34851 = t30540 * t1549;
    (t34836, t34839, t34841, t34844, t34849, t34851)
}
