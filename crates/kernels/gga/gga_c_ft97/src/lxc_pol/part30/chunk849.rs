//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 849/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk849<F: Float>(t1466: F, t34329: F, t681: F, t33966: F, t683: F, t2399: F, t7586: F, t34311: F, t92: F, t34057: F, t7613: F, t28658: F, t7203: F, t2691: F, t33939: F, t4113: F) -> (F, F, F, F, F, F, F, F, F) {
    let t142647 = t1466 * t681 * t34329;
    let t142653 = t683 * t33966;
    let t142662 = 4.0 / 27.0 * t1466 * t2399 * t7586;
    let t142663 = t34311 * t92;
    let t142677 = t1466 * t681 * t34057;
    let t142688 = 2.0 / 27.0 * t1466 * t2399 * t7613;
    let t142696 = t28658 * t7203;
    let t142697 = t2691 * t142696;
    let t142704 = t4113 * t33939 * t7203;
    (t142647, t142653, t142662, t142663, t142677, t142688, t142696, t142697, t142704)
}
