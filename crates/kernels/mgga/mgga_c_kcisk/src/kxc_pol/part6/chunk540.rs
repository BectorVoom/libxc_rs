//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 540/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk540<F: Float>(t5061: F, t5320: F, t747: F, t79: F, t1965: F, t2597: F, t1676: F, t2386: F, t2394: F, t4790: F, t240: F, t260: F, t604: F, t67: F) -> (F, F, F, F, F, F, F) {
    let t7429 = t5061 * t5320;
    let t7430 = t79 * t747;
    let t7467 = t2597 * t1965;
    let t7498 = t2386 * t1676;
    let t7509 = t2394 * t4790;
    let t7517 = t240 * t2386;
    let t7567 = t260 * t67 * t604;
    (t7429, t7430, t7467, t7498, t7509, t7517, t7567)
}
