//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 835/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk835<F: Float>(t12467: F, t3: F, t213: F, t12476: F, t2862: F, t12485: F, t817: F, t5680: F, t12468: F, t12469: F, t12473: F, t12480: F, t15: F, t2863: F, t2866: F, t3092: F, t3096: F, t818: F, t947: F) -> (F, F, F, F) {
    let t12488 = t12467 * t3;
    let t12489 = t12488 * t213;
    let t12491 = t2862 * t12476;
    let t12493 = t817 * t12485;
    let t12496 = -0.26426666666666666667e-1 * t12489 + 0.17617777777777777778e-1 * t12491 - 0.20554074074074074074e-1 * t12493 - 0.12841111111111111111e-1 * t5680;
    let t12499 = -t12468 * t12469 / 3.0 - t12473 * t2863 / 6.0 + 2.0 / 9.0 * t3092 * t12476 - t12480 * t818 / 4.0 + t3096 * t2866 / 3.0 - 7.0 / 27.0 * t947 * t12485 + t15 * t12496 / 2.0;
    (t12489, t12491, t12493, t12499)
}
