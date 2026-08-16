//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1000/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1000<F: Float>(t591: F, t6718: F, t6722: F, t208: F, t213: F, t579: F, t6716: F, t588: F, t6717: F, t97: F, t1696: F, t2414: F) -> (F, F, F, F, F) {
    let t18257 = t6718 * t591;
    let t18259 = t6722 * t591;
    let t18274 = t6716 * t579 * t208 * t213;
    let t18277 = t6717 * t97 * t588;
    let t18281 = t2414 * t1696 * t208 * t213;
    (t18257, t18259, t18274, t18277, t18281)
}
