//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 850/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk850<F: Float>(t1272: F, t4913: F, t1239: F, t342: F, t740: F, t934: F, t3576: F, t28: F, t3: F, t37: F, t27: F, t4238: F, t55: F) -> (F, F, F, F, F, F) {
    let t8293 = F::new(2.9018074074074076) * t1272 * t4913;
    let t8295 = F::new(5.773876543209877) * t1239 * t4913;
    let t8305 = t934 * t740 * t342;
    let t8306 = t3576 * t8305;
    let t8333 = F::new(1.0) / t37 / t28 / t3 / F::new(48.0);
    let t8337 = t4238 * t27 * t55;
    (t8293, t8295, t8305, t8306, t8333, t8337)
}
