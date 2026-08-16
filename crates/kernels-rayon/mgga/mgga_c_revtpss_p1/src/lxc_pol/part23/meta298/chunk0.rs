//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1545/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1545(t3154: f64, t999: f64, t1086: f64, t3046: f64, t3090: f64, t3316: f64, t994: f64, t4891: f64) -> (f64, f64, f64, f64, f64) {
    let t11860 = t3154 * t999;
    let t11865 = t3046 * t1086;
    let t11866 = t11865 * t3090;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    (t11860, t11865, t11866, t11874, t11875)
}
