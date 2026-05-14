//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1350/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1350<F: Float>(t27423: F, t32278: F, t1493: F, t8232: F, t33643: F, t33658: F, t27180: F, t394: F, t9492: F, t27188: F, t27101: F, t32255: F, t34863: F, t33676: F, t6318: F, t27172: F, t33655: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119714 = t32278 * t27423;
    let t119716 = t8232 * t1493;
    let t119718 = t33643 * t33658;
    let t119720 = t27180 * t394;
    let t119721 = t119720 * t9492;
    let t119723 = t32278 * t27188;
    let t119725 = t32278 * t27101;
    let t119727 = t32255 * t34863;
    let t119729 = t33676 * t6318;
    let t119731 = t33655 * t27172;
    (t119714, t119716, t119718, t119721, t119723, t119725, t119727, t119729, t119731)
}
