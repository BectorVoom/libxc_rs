//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1118/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1118(t28616: f64, t33414: f64, t150590: f64, t31462: f64, t33893: f64, t35367: f64, t1196: f64, t7464: f64, t142832: f64, t820: f64, t6789: f64, t817: f64) -> (f64, f64, f64, f64, f64) {
    let t153025 = t33414 * t28616;
    let t153035 = t31462 * t150590;
    let t153039 = t35367 * t33893;
    let t153042 = t7464 * t1196;
    let t153044 = t142832 * t153042 * t820;
    let t153047 = t817 * t6789;
    (t153025, t153035, t153039, t153044, t153047)
}
