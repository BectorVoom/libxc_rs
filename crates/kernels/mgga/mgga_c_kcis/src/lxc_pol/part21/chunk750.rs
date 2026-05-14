//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 750/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk750<F: Float>(t3214: F, t9429: F, t3209: F, t982: F, t2865: F, t359: F, t169: F, t2843: F) -> (F, F, F, F) {
    let t9430 = t9429 * t3214;
    let t9438 = t3209 * t982;
    let t9476 = t2865 * t359;
    let t9494 = 1.0 / t2843 / t169;
    (t9430, t9438, t9476, t9494)
}
