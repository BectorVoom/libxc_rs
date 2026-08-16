//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 887/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk887(t124: f64, t22813: f64, t800: f64, t1883: f64, t22079: f64, t5673: f64, t1872: f64, t6816: f64, t22046: f64, t3936: f64, t6869: f64, t543: f64, t6836: f64) -> (f64, f64, f64, f64, f64) {
    let t22876 = t124 * t22813;
    let t22877 = t800 * t22876;
    let t22881 = t5673 * t22079 * t1883;
    let t22886 = t800 * t1872 * t6816;
    let t22890 = t3936 * t22046 * t6869;
    let t22893 = t543 * t6836;
    (t22877, t22881, t22886, t22890, t22893)
}
