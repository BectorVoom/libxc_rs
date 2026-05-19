//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1069/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1069<F: Float>(t121140: F, t121142: F, t25898: F, t7063: F, t8578: F, t4104: F, t550: F, t561: F, t9794: F, t2453: F, t8571: F, t240: F, t27: F, t545: F) -> (F, F, F, F, F, F, F) {
    let t121144 = F::cast_from(0.50779446784275991476e-2_f64) * t121140 * t121142;
    let t121146 = t7063 * t8578 * t25898;
    let t121147 = t121146 * t4104;
    let t121165 = t550 * t561;
    let t121166 = t9794 * t121165;
    let t121167 = t2453 * t8571 * t121166;
    let t121168 = F::cast_from(0.13386901839087538753e-4_f64) * t121167;
    let t121173 = t545 * t27 * t240;
    (t121144, t121146, t121147, t121165, t121166, t121168, t121173)
}
