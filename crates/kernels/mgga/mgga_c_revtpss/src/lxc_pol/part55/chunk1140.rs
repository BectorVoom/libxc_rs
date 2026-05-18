//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1140/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1140<F: Float>(t121116: F, t32213: F, t125: F, t4075: F, t121035: F, t25875: F, t550: F, t561: F, t9794: F, t2453: F, t8571: F, t240: F, t27: F, t545: F) -> (F, F, F, F, F, F, F) {
    let t121117 = t121116 * t32213;
    let t121118 = F::new(0.263521689745817692e-2) * t121117;
    let t121126 = t125 * t4075;
    let t121131 = t25875 * t121035;
    let t121165 = t550 * t561;
    let t121166 = t9794 * t121165;
    let t121167 = t2453 * t8571 * t121166;
    let t121173 = t545 * t27 * t240;
    (t121118, t121126, t121131, t121165, t121166, t121167, t121173)
}
