//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1100/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1100(t2194: f64, t9981: f64, t2012: f64, t7809: f64, t9801: f64, t2679: f64, t7696: f64, t9800: f64, t2624: f64, t7383: f64, t1391: f64, t825: f64, t9850: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28659 = t2194 * t9981;
    let t28673 = t2012 * t7809;
    let t28675 = 0.38342925953920749676e1_f64 * t28673 * t9801;
    let t28678 = 0.38342925953920749676e1_f64 * t9800 * t7696 * t2679;
    let t28681 = 0.19171462976960374838e1_f64 * t9800 * t2624 * t7383;
    let t28683 = t825 * t1391 * t9850;
    (t28659, t28673, t28675, t28678, t28681, t28683)
}
