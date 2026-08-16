//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2074/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2074(t7063: f64, t94878: f64, t7286: f64, t7289: f64, t94810: f64, t26054: f64, t9686: f64, t25877: f64, t94801: f64, t25881: f64, t1419: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94879 = t7063 * t94878;
    let t94880 = t94879 * t7286;
    let t94882 = t7289 * t94810;
    let t94884 = t26054 * t9686;
    let t94886 = t94801 * t25877;
    let t94887 = t94886 * t25881;
    let t94889 = t786 * t1419;
    (t94880, t94882, t94884, t94886, t94887, t94889)
}
