//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 858/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk858<F: Float>(t3709: F, t682: F, t696: F, t8599: F, t1025: F, t1035: F, t3666: F, t3669: F, t3952: F, t687: F, t3947: F, t654: F) -> (F, F, F, F) {
    let t8603 = F::new(14.03573669432315) * t696 * t3709 * t8599 * t682;
    let t8610 = F::new(3103.560775156404) * t3666 * t1035 * t3669 * t1025;
    let t8612 = F::new(480.0) * t3952 * t687;
    let t8614 = t3947 * t654;
    (t8603, t8610, t8612, t8614)
}
