//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2355/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2355<F: Float>(t96667: F, t96669: F, t96671: F, t96673: F, t96675: F, t96677: F, t96679: F, t96681: F, t96685: F, t96704: F, t96706: F, t96708: F, t96711: F, t96731: F) -> F {
    let t104996 = t96667 + t96669 + t96671 + t96673 + t96675 + t96677 + t96679 + t96681 + t96685 + t96704 + t96706 + t96708 + t96711 + t96731;
    t104996
}
