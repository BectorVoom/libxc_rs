//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 729/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk729<F: Float>(t1977: F, t4606: F, t3518: F, t739: F, t940: F, t3536: F, t11: F, t3476: F, t1243: F, t1245: F, t34: F, t348: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
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
