//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1360/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1360(t210: f64, t24810: f64, t24848: f64, t1090: f64, t24815: f64, t11624: f64, t11882: f64, t11904: f64, t11914: f64, t11915: f64, t24660: f64, t24776: f64, t24812: f64, t24820: f64, t24821: f64, t24849: f64, t24851: f64, t24858: f64, t24863: f64, t27532: f64, t3243: f64, t3610: f64, t3611: f64, t3612: f64, t3620: f64, t52531: f64, t7283: f64, t7327: f64, t7376: f64, t85836: f64, t85963: f64, t86015: f64, t86016: f64, t86020: f64, t86022: f64, t86023: f64, t86032: f64) -> (f64, f64) {
    let t86036 = t24810 * t210;
    let t86037 = t86036 * t24848;
    let t86039 = t24815 * t1090;
    let t86051 = 0.10966227112321509577e-1_f64 * t7283 * t24776 * t24858 * t3243 - 0.82246703342411321826e-2_f64 * t24849 * t24851 * t52531 * t7376 - 0.16449340668482264365e-1_f64 * t24849 * t86015 * t86016 - 0.16449340668482264365e-1_f64 * t86020 + 0.82246703342411321825e-2_f64 * t85963 * t86022 * t11882 * t86023 - 0.24674011002723396548e-1_f64 * t24812 * t24820 * t11624 * t24821 + 6.0_f64 * t3610 * t86032 * t3612 - 0.16449340668482264365e-1_f64 * t86037 * t24660 * t3611 * t86039 - 0.82246703342411321826e-2_f64 * t24849 * t7327 * t3620 * t27532 + 6.0_f64 * t11904 * t24863 + t11914 * t85836 * t11915;
    (t86037, t86051)
}
