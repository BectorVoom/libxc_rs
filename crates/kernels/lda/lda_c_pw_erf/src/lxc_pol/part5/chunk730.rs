//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 730/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk730<F: Float>(t494: F, t793: F, t184: F, t786: F, t2468: F, t565: F, t2114: F, t2505: F, t1298: F, t2849: F, t462: F, t198: F) -> (F, F, F, F, F, F, F, F) {
    let t6579 = t494 * t793;
    let t6580 = t6579 * t184;
    let t6582 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t6580 * t786;
    let t6584 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t565 * t2468;
    let t6586 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2114 * t2505;
    let t6588 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1298 * t2505;
    let t6590 = -t462 - F::cast_from(3.0_f64) * t2849;
    let t6591 = t198 * t6590;
    (t6579, t6580, t6582, t6584, t6586, t6588, t6590, t6591)
}
