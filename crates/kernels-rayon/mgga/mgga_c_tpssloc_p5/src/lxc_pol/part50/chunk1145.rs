//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1145/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1145(t1036: f64, t30824: f64, t23472: f64, t6746: f64, t6753: f64, t1940: f64, t23478: f64, t23477: f64, t30829: f64, t3103: f64, t30828: f64, t3113: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113361 = t30824 * t1036;
    let t113372 = t23472 * t6753 * t6746;
    let t113380 = t23478 * t1940;
    let t113381 = t23477 * t113380;
    let t113388 = t30829 * t3103;
    let t113392 = t3113 * t30828;
    (t113361, t113372, t113380, t113381, t113388, t113392)
}
