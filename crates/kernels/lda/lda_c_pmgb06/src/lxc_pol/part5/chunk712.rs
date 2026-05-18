//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 712/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk712<F: Float>(t493: F, t6533: F, t1982: F, t1988: F, t1981: F, t1444: F, t2466: F, t1450: F, t2465: F, t498: F, t5974: F, t496: F) -> (F, F, F, F, F, F, F, F) {
    let t6535 = F::new(2.0) / F::new(45.0) * t493 * t6533;
    let t6536 = t1988 * t1982;
    let t6538 = F::new(4.0) / F::new(45.0) * t1981 * t6536;
    let t6540 = t1444 * t2466 / F::new(45.0);
    let t6541 = t1450 * t2465;
    let t6543 = t493 * t6541 / F::new(45.0);
    let t6544 = t498 * t5974;
    let t6545 = t496 * t6544;
    (t6535, t6536, t6538, t6540, t6541, t6543, t6544, t6545)
}
