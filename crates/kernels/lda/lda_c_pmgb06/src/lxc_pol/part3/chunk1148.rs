//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1148/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1148<F: Float>(t132: F, t1547: F, t2042: F, t1963: F, t3213: F, t1423: F, t4772: F, t1710: F, t801: F, t446: F, t3259: F, t813: F) -> (F, F, F, F, F) {
    let t13706 = t132 * t1547 * t2042;
    let t13707 = t13706 / F::new(45.0);
    let t13708 = t3213 * t1963;
    let t13709 = F::new(2.0) / F::new(135.0) * t13708;
    let t13710 = t1423 * t4772;
    let t13711 = F::new(4.0) / F::new(45.0) * t13710;
    let t13712 = t801 * t1710;
    let t13713 = t13712 * t446;
    let t13714 = F::new(2.0) / F::new(135.0) * t13713;
    let t13715 = t3259 * t813;
    (t13707, t13709, t13711, t13714, t13715)
}
