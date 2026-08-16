//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1763/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1763(t1388: f64, t5356: f64, t1351: f64, t5187: f64, t19735: f64, t1352: f64, t5286: f64, t1799: f64, t3698: f64, t4303: f64, t776: f64, t1484: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56404 = t1388 * t5356;
    let t56805 = t5187 * t1351;
    let t57554 = t19735 * t1351;
    let t57643 = t1352 * t5286;
    let t57802 = t1799 * t3698;
    let t57893 = t776 * t4303;
    let t57911 = t2752 * t1484;
    (t56404, t56805, t57554, t57643, t57802, t57893, t57911)
}
