//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1294/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1294<F: Float>(t198: F, t39419: F, t39422: F, t39424: F, t39426: F, t39429: F, t39432: F, t39434: F, t39437: F, t39439: F, t39442: F, t39476: F, t39483: F, t39520: F, t765: F) -> F {
    let t39521 = F::cast_from(3.0_f64) * t198 * t39476 * t765 - t39419 - t39422 - t39424 - t39426 - t39429 - t39432 + t39434 + t39437 + t39439 + t39442 - t39483 + t39520;
    t39521
}
