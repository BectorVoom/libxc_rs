//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1153/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1153(t23133: f64, t5624: f64, t1516: f64, t87340: f64, t16673: f64, t6620: f64, t23083: f64, t28375: f64, t28396: f64, t81835: f64, t23110: f64, t23185: f64, t28321: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98828 = t23133 * t5624;
    let t98830 = t87340 * t1516;
    let t98832 = t16673 * t6620;
    let t98836 = t23083 * t28375;
    let t98838 = t81835 * t28396;
    let t98884 = t23185 * t23110 * t28321;
    (t98828, t98830, t98832, t98836, t98838, t98884)
}
