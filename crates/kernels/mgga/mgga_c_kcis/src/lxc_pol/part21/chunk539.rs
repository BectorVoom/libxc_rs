//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 539/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk539<F: Float>(t3463: F, t3228: F, t359: F, t376: F, t1166: F, t1176: F, t1180: F, t1094: F, t1164: F, t1172: F, t3177: F, t381: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t3464 = t3463 * sigma0;
    let t3465 = t359 * t3228;
    let t3466 = t376 * t3465;
    let t3467 = t3464 * t3466;
    let t3469 = t1166 * t1176;
    let t3471 = t1166 * t1180;
    let t3473 = t1164 * t1094;
    let t3474 = t3473 * sigma0;
    let t3475 = t3474 * t1172;
    let t3477 = t3177 * t381;
    (t3466, t3467, t3469, t3471, t3473, t3474, t3475, t3477)
}
