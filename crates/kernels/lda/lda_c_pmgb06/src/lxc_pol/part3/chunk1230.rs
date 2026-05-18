//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1230/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1230<F: Float>(t10577: F, t4354: F, t2257: F, t4042: F, t10544: F, t10558: F, t10570: F, t10578: F, t10594: F, t1312: F, t1316: F, t14567: F, t14570: F, t14571: F, t14575: F, t14587: F, t14593: F, t346: F, t4013: F, t4045: F, t4231: F, t4398: F, t4414: F, t5583: F, t6018: F, t790: F) -> F {
    let t14596 = t10577 * t4354;
    let t14601 = t2257 * t4042;
    let t14606 = -F::new(18.0) * t5583 * t10578 + F::new(0.05987117005127304) * t14567 + t14570 + F::new(0.05987117005127304) * t14571 + F::new(0.0001639671923854359) * t14575 + F::new(6.0) * t1316 * t1312 * t4414 - F::new(6.0) * t346 * t4398 * t4013 + F::new(3.0) * t1316 * t790 * t10558 + F::new(6.0) * t4231 * t14587 - F::new(18.0) * t6018 * t10570 - F::new(3.0) * t4231 * t14593 - F::new(6.0) * t4231 * t14596 + F::new(18.0) * t6018 * t10544 + F::new(6.0) * t346 * t14601 * t4045 - F::new(0.054045904796391424) * t10594;
    t14606
}
