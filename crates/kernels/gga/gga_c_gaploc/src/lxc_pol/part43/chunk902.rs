//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 902/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk902<F: Float>(t14517: F, t1955: F, t42470: F, t42473: F, t42475: F, t42481: F, t42483: F, t42485: F, t42487: F, t42491: F, t42494: F, t42496: F, t42501: F, t42503: F, t43346: F, t43353: F, t43355: F, t44194: F, t50811: F) -> (F,) {
    let t51063 = -t14517 * t1955 + t42470 + t42473 - t42475 + t42481 - t42483 + t42485 - t42487 - t42491 - t42494 - t42496 - t42501 - t42503 + t43346 + t43353 - t43355 - t44194 + t50811;
    (t51063,)
}
