//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 702/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk702<F: Float>(t1085: F, t4397: F, t2743: F, t1067: F, t749: F, t1070: F, t1034: F, t748: F, t40: F, t1064: F, t4383: F, t85: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4398 = t4397 * t1085;
    let t4399 = F::new(0.010843580882781523) * t4398;
    let t4400 = F::new(0.5848223397455204) * t2743;
    let t4401 = t1067 * t749;
    let t4402 = F::new(12.0) * t4401;
    let t4403 = t1070 * t749;
    let t4404 = F::new(32.0) * t4403;
    let t4405 = t748 * t1034;
    let t4406 = t40 * t4405;
    let t4408 = t1064 * t749;
    let t4409 = F::new(20.0) * t4408;
    let t4410 = t4383 * t85;
    (t4398, t4399, t4400, t4401, t4402, t4403, t4404, t4405, t4406, t4408, t4409, t4410)
}
