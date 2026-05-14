//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1122/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1122<F: Float>(t53: F, t925: F, t100521: F, t100524: F, t115370: F, t115379: F, t115381: F, t115385: F, t115389: F, t115397: F, t115405: F, t115410: F, t1603: F, t22522: F, t22534: F, t22568: F, t22583: F, t22585: F, t22613: F, t22619: F, t22718: F, t22797: F, t22798: F, t29468: F, t29490: F, t29515: F, t2993: F, t379: F, t5513: F, t5569: F, t5570: F, t73: F, t92476: F, t930: F, t93055: F, t93078: F, t93129: F) -> (F,) {
    let t115418 = t925 * t53;
    let t115423 = 0.12768721675925925926e-1 * t22522 * t5570 * t115370 * t379 - 0.59387071557258112888e-3 * t5569 * t22568 * t29490 + 0.7423383944657264111e-4 * t115379 - 0.89080607335887169332e-4 * t22613 * t73 * t115381 + 0.89080607335887169332e-4 * t22619 * t73 * t115385 - 0.29673063867321838428e-4 * t22534 * t73 * t115389 - 0.87941772264679191254e-7 * t93055 * t22797 * t29468 * t22798 + 0.24710505058474293383e-6 * t93078 * t73 * t115397 - 0.23254900946437792e-1 * t1603 * t22718 * t29515 - 0.23254900946437792e-1 * t1603 * t5513 * t115405 - t100521 - t100524 - 0.29693535778629056444e-4 * t22583 * t92476 * t115410 - 0.29693535778629056444e-3 * t22583 * t22585 * t930 * t2993 + 0.35200977868053026979e-5 * t93129 * t22585 * t930 * t115418;
    (t115423,)
}
