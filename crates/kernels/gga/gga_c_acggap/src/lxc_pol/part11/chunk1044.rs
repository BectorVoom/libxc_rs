//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1044/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1044<F: Float>(t30248: F, t537: F, t7637: F, t8859: F, t1576: F, t7614: F, t13299: F, t33952: F, t33954: F, t15386: F, t31443: F, t35704: F, t17912: F, t33953: F, t5207: F, t142: F, t5160: F, t7436: F) -> (F, F, F, F, F, F, F) {
    let t36236 = t30248 * t537;
    let t36238 = t7637 * t8859;
    let t36240 = t7614 * t1576;
    let t36243 = t33952 * t13299 * t33954;
    let t36246 = t31443 * t15386 * t35704;
    let t36250 = t31443 * t17912 * t33953 * t5207;
    let t36253 = t7436 * t142 * t5160;
    (t36236, t36238, t36240, t36243, t36246, t36250, t36253)
}
