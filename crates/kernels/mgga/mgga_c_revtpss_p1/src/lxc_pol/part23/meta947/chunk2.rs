//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3130/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3130<F: Float>(t5245: F, t5819: F, t81128: F, t81130: F, t81132: F, t81134: F, t81136: F, t81138: F, t81145: F, t81148: F, t81150: F, t81152: F, t81254: F, t81257: F, t81259: F, t81261: F, t81264: F, t81266: F, t81307: F, t81309: F, t81313: F, t81315: F) -> (F, F) {
    let t82368 = t5819 * t5245;
    let t82385 = t81128 + t81130 + t81132 + t81134 + t81136 - t81138 - t81145 + t81148 - t81150 + t81152 + t81254 - t81257 - t81259 + t81261 + t81264 - t81266 - t81307 + t81309 - t81313 - t81315;
    (t82368, t82385)
}
