//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2250/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2250(t25068: f64, t4257: f64, t16853: f64, t6621: f64, t16946: f64, t16951: f64, t23053: f64, t5619: f64, t23083: f64, t28356: f64, t25093: f64, t7496: f64, t87504: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98715 = t25068 * t4257;
    let t98717 = t6621 * t16853;
    let t98719 = t6621 * t16946;
    let t98721 = t6621 * t16951;
    let t98723 = t23053 * t5619;
    let t98725 = t23083 * t28356;
    let t98728 = t87504 * t7496 * t25093;
    (t98715, t98717, t98719, t98721, t98723, t98725, t98728)
}
