//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1286/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1286(t10143: f64, t7540: f64, t1649: f64, t2752: f64, t1410: f64, t9239: f64, t22716: f64, t7697: f64, t7692: f64, t81186: f64, t1834: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87975 = t7540 * t10143;
    let t89992 = t2752 * t1649;
    let t90137 = t9239 * t1410;
    let t90503 = t22716 * t7697;
    let t90521 = t81186 * t7692;
    let t90544 = t794 * t1834;
    (t87975, t89992, t90137, t90503, t90521, t90544)
}
