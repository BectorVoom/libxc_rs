//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1182/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1182(t31285: f64, t3941: f64, t1873: f64, t649: f64, t6534: f64, t89: f64, t645: f64, t8513: f64, t8514: f64, t1862: f64, t31: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31286 = t3941 * t31285;
    let t31287 = 27.0_f64 * t31286;
    let t31537 = t649 * t1873;
    let t31540 = t89 * t6534;
    let t31677 = t8513 * t8514 * t645;
    let t31682 = t1862 * t31;
    let t31683 = t31682 * t607;
    (t31287, t31537, t31540, t31677, t31682, t31683)
}
