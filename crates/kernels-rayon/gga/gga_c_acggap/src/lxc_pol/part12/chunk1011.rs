//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1011/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1011(t30937: f64, t8614: f64, t30934: f64, t8597: f64, t2264: f64, t30797: f64, t7839: f64, t8518: f64, t8522: f64, t31699: f64, t8526: f64, t4713: f64, t7822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34029 = t30937 * t8614;
    let t34031 = t30934 * t8597;
    let t34033 = t30797 * t2264;
    let t34035 = t7839 * t8518;
    let t34037 = t7839 * t8522;
    let t34039 = t31699 * t8526;
    let t34041 = t7822 * t4713;
    (t34029, t34031, t34033, t34035, t34037, t34039, t34041)
}
