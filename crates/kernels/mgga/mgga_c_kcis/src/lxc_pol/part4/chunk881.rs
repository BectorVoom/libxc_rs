//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 881/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk881<F: Float>(t3214: F, t9429: F, t3209: F, t982: F, t2865: F, t359: F, t169: F, t2843: F, t1131: F, t3201: F, t2861: F, t3192: F, t3179: F, t251: F, t88: F, t304: F, t86: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9430 = t9429 * t3214;
    let t9438 = t3209 * t982;
    let t9476 = t2865 * t359;
    let t9494 = 1.0 / t2843 / t169;
    let t9517 = t3201 * t1131;
    let t9522 = t2861 * t3192;
    let t9524 = t2861 * t3179;
    let t9526 = t88 * t251;
    let t9528 = t86 * t9526 * t304;
    (t9430, t9438, t9476, t9494, t9517, t9522, t9524, t9526, t9528)
}
