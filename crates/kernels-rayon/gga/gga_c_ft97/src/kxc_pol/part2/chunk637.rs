//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 637/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk637(t1771: f64, t471: f64, t1554: f64, t369: f64, t2: f64, t1775: f64, t1788: f64, t1793: f64, t631: f64, t637: f64, t7242: f64, t96: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8302 = t1771 * t471;
    let t8326 = t1554 * t369;
    let t8327 = t8326 * t2;
    let t8331 = t1775 * t1788;
    let t8333 = t1775 * t1793;
    let t8345 = 1.0_f64 / t96 / t631 / t637 / t369 / t7242 / 4.0_f64;
    (t8302, t8326, t8327, t8331, t8333, t8345)
}
