//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1045/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1045<F: Float>(t12131: F, t16316: F, t16321: F, t16323: F, t16325: F, t16328: F, t16331: F, t16334: F, t16338: F, t16341: F, t16345: F, t1686: F, t18472: F, t18540: F, t18563: F, t18598: F, t18646: F, t1987: F, t2396: F, t240: F, t4764: F, t4783: F, t4791: F, t5423: F, t6881: F, t7517: F) -> (F,) {
    let t18650 = 0.11696446794910408142e1 * t1987 * t16316 - 0.11696446794910408142e1 * t18472 * t1686 - 0.58482233974552040708e0 * t7517 * t4783 - 0.17315755899375863299e2 * t7517 * t4791 - 0.58482233974552040708e0 * t12131 * t2396 + t16321 - t16323 + t16325 - t16328 - t16331 - t16334 + t16338 + t16341 + t16345 - 0.34631511798751726598e2 * t5423 * t6881 + 0.11696446794910408142e1 * t7517 * t4764 + t240 * (t18540 + t18563 + t18598 + t18646);
    (t18650,)
}
