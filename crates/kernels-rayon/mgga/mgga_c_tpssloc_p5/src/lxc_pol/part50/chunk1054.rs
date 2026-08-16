//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1054/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1054(t31003: f64, t9239: f64, t645: f64, t8307: f64, t8513: f64, t33: f64, t8303: f64, t2240: f64, t31: f64, t607: f64, t1862: f64, t8301: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31004 = t9239 * t31003;
    let t31005 = t8307 * t645;
    let t31006 = t8513 * t31005;
    let t31009 = t33 * t8303;
    let t31010 = t2240 * t31009;
    let t31011 = t8307 * t31;
    let t31013 = t8513 * t31011 * t607;
    let t31016 = t8301 * t1862;
    (t31004, t31006, t31009, t31010, t31011, t31013, t31016)
}
