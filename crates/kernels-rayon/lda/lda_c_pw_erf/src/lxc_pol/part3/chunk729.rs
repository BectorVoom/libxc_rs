//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 729/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk729(t1977: f64, t4606: f64, t3518: f64, t739: f64, t940: f64, t3536: f64, t11: f64, t3476: f64, t1243: f64, t1245: f64, t34: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4607 = t4606 * t1977;
    let t4609 = t3518 * t739;
    let t4610 = t4609 * t940;
    let t4611 = t3536 * t4610;
    let t4612 = t11 * t4611;
    let t4614 = t3476 * t739;
    let t4615 = t4614 * t940;
    let t4616 = t1243 * t4615;
    let t4617 = t11 * t4616;
    let t4619 = t1245 * t34;
    let t4620 = t4619 * t348;
    (t4607, t4609, t4610, t4611, t4612, t4614, t4615, t4616, t4617, t4619, t4620)
}
