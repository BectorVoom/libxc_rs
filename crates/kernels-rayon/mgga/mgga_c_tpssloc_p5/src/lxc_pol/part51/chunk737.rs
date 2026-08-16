//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 737/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk737(t1987: f64, t794: f64, t6897: f64, t1372: f64, t225: f64, t567: f64, t214: f64, t1985: f64, t1377: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6898 = t794 * t1987;
    let t6899 = t6897 * t6898;
    let t6900 = 0.41123351671205660912e-2_f64 * t6899;
    let t6902 = t1372 * t225 * t567;
    let t6903 = t214 * t6902;
    let t6904 = t1985 * t6903;
    let t6906 = t225 * t1377;
    (t6898, t6899, t6900, t6902, t6903, t6904, t6906)
}
