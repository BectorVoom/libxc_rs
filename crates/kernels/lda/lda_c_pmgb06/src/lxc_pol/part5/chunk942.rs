//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 942/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk942<F: Float>(t12368: F, t2057: F, t955: F, t2054: F, t12535: F, t495: F, t5065: F, t132: F, t1547: F, t2042: F, t1963: F, t3213: F) -> (F, F, F, F, F, F) {
    let t13595 = F::new(0.03199259259259259) * t12368;
    let t13619 = t955 * t2057;
    let t13621 = t955 * t2054;
    let t13672 = t5065 * t12535 * t495;
    let t13706 = t132 * t1547 * t2042;
    let t13707 = t13706 / F::new(45.0);
    let t13708 = t3213 * t1963;
    (t13595, t13619, t13621, t13672, t13707, t13708)
}
