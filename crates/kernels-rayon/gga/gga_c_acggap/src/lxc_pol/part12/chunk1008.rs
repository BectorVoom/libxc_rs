//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1008/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1008(t1181: f64, t22778: f64, t7564: f64, t8600: f64, t1983: f64, t30692: f64, t5720: f64, t7586: f64, t7839: f64, t8779: f64, t4991: f64, t7822: f64) -> (f64, f64, f64, f64) {
    let t33990 = t7564 * t1181 * t8600 * t22778;
    let t33994 = t30692 * t7586 * t1983 * t5720;
    let t33996 = t7839 * t8779;
    let t33998 = t7822 * t4991;
    (t33990, t33994, t33996, t33998)
}
