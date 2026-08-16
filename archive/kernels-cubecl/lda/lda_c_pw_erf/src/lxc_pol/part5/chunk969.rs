//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 969/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk969<F: Float>(t1370: F, t3604: F, t3586: F, t3589: F, t1351: F, t213: F, t573: F, t1484: F, t2058: F, t933: F, t2055: F, t1950: F, t925: F) -> (F, F, F, F, F, F, F, F) {
    let t13635 = t1370 * t3604;
    let t13639 = t3586 * t3589;
    let t13643 = t213 * t1351;
    let t13653 = t573 * t3604;
    let t13657 = t1484 * t3589;
    let t13661 = t933 * t2058;
    let t13663 = t933 * t2055;
    let t13710 = t925 * t1950;
    (t13635, t13639, t13643, t13653, t13657, t13661, t13663, t13710)
}
