//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 942/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk942<F: Float>(t11371: F, t1051: F, t4393: F, t1070: F, t1799: F, t8357: F, t344: F, t5685: F, t1: F, t1750: F, t1755: F, t2316: F) -> (F, F, F, F, F, F) {
    let t11372 = F::new(51.94726769812759) * t11371;
    let t11373 = t4393 * t1051;
    let t11374 = F::new(1.7544670192365612) * t11373;
    let t11376 = F::new(96.0) * t1070 * t1799;
    let t11382 = F::new(8.0) * t8357;
    let t11387 = t344 * t5685;
    let t11388 = F::new(24.0) * t11387;
    let t11397 = t2316 * t1750 * t1 * t1755;
    (t11372, t11374, t11376, t11382, t11388, t11397)
}
