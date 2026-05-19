//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 783/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk783<F: Float>(t7276: F, t7325: F, t2722: F, t787: F, t2730: F, t2448: F, t769: F, t2247: F, t2248: F, t3505: F, t3517: F, t3525: F, t3643: F, t5852: F, t69: F, t7069: F, t7071: F, t7261: F, t7262: F, t7270: F, t7271: F, t7274: F, t7283: F, t7309: F, t7318: F, t7322: F) -> (F, F, F, F, F) {
    let t7326 = t7276 + t7325;
    let t7334 = t2722 * t787;
    let t7337 = t787 * t2730;
    let t7344 = t769 * t2448;
    let t7351 = -F::new(5.172765) * t7069 + F::new(1.724255) * t7071 - F::new(20.69106) * t69 * t7322 + F::new(15.518295) * t2247 * t2248 * t7344 - t7261 + t7262 - t7283 + t7270 - F::new(1.724255) * t69 * t7318 - t7309 - F::cast_from(2.2990066666666666_f64) * t5852 + t7274 - t7271 - t3643 - t3505 - t3517 + t3525;
    (t7326, t7334, t7337, t7344, t7351)
}
