//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1014/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1014(t30268: f64, t8775: f64, t30105: f64, t8952: f64, t7839: f64, t8739: f64, t1181: f64, t22778: f64, t7564: f64, t8600: f64, t1983: f64, t30692: f64, t5720: f64, t7586: f64) -> (f64, f64, f64, f64, f64) {
    let t33982 = t30268 * t8775;
    let t33983 = 0.64311027177104605458e-2_f64 * t33982;
    let t33984 = t30105 * t8952;
    let t33986 = t7839 * t8739;
    let t33987 = 0.62896184579208304136e-3_f64 * t33986;
    let t33990 = t7564 * t1181 * t8600 * t22778;
    let t33994 = t30692 * t7586 * t1983 * t5720;
    (t33983, t33984, t33987, t33990, t33994)
}
