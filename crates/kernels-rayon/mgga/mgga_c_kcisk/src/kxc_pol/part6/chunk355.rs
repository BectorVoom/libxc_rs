//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 355/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk355(t2225: f64, t457: f64, t1419: f64, t1421: f64, t2110: f64, t2218: f64, t2222: f64, t338: f64, t456: f64) -> (f64, f64) {
    let t2226 = t457 * t2225;
    let t2231 = t1419 + 0.65704296666666666667e-3_f64 * t1421 * t2218 + 0.1478346675e-2_f64 * t456 * t2222 - 0.98556445e-3_f64 * t456 * t2226 - 4.0_f64 * t338 * t2110;
    (t2226, t2231)
}
