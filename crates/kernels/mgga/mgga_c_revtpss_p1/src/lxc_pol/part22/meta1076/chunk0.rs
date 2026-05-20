//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3856/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3856<F: Float>(t47093: F, t39989: F, t47084: F, t47086: F, t47088: F, t47092: F, t47096: F, t74114: F, t74115: F, t74116: F, t74117: F, t74119: F, t74120: F, t74121: F, t74122: F, t74123: F, t74124: F, t74125: F) -> (F, F) {
    let t74126 = F::cast_from(0.20779030926817756511e3_f64) * t47093;
    let t74127 = -t74114 + t74115 + t74116 - t74117 + t74119 - t74120 - t47084 - t74121 + t74122 + t74123 + t74124 - t39989 - t47086 + t47088 - t74125 + t47092 + t74126 - t47096;
    (t74126, t74127)
}
