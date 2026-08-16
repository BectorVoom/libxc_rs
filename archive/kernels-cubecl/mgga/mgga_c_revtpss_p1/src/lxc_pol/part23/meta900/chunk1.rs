//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2862/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2862<F: Float>(t50901: F, t40076: F, t40079: F, t40194: F, t40198: F, t77036: F, t77038: F, t77039: F, t77040: F, t77041: F, t77045: F, t77048: F, t77051: F, t77053: F, t77056: F, t77058: F, t77059: F) -> (F, F) {
    let t77060 = F::cast_from(0.97592231702715658578e-1_f64) * t50901;
    let t77061 = t77036 + t77038 + t77039 + t77040 + t77041 + t77045 - t77048 + t77051 + t77053 + t77056 + t77058 + t40076 - t40079 + t40194 + t40198 - t77059 - t77060;
    (t77060, t77061)
}
