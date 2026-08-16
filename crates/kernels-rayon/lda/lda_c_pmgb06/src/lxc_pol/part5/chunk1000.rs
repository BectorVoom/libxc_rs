//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1000/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1000(t591: f64, t6718: f64, t6722: f64, t208: f64, t213: f64, t579: f64, t6716: f64, t588: f64, t6717: f64, t97: f64, t1696: f64, t2414: f64) -> (f64, f64, f64, f64, f64) {
    let t18257 = t6718 * t591;
    let t18259 = t6722 * t591;
    let t18274 = t6716 * t579 * t208 * t213;
    let t18277 = t6717 * t97 * t588;
    let t18281 = t2414 * t1696 * t208 * t213;
    (t18257, t18259, t18274, t18277, t18281)
}
