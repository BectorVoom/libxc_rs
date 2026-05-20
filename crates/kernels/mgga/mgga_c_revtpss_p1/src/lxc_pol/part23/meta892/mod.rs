//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta892 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2847;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta892<F: Float>(t76911: F, t76929: F, t150: F, t190: F, t162: F, t187: F, t61020: F, t49866: F, t39423: F, t39425: F, t39433: F, t39438: F, t61090: F, t39419: F, t39422: F, t39429: F, t39432: F, t39442: F, t49877: F, t76890: F, t76893: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t76932, t76935, t76936, t76937, t76938, t76939, t76940, t76941) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2847::<F>(t76911, t76929, t150, t190, t162, t187, t61020, t49866, t39423, t39425, t39433, t39438);
        let (t76942, t76943) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2848::<F>(t61090, t39419, t39422, t39429, t39432, t39442, t49877, t76890, t76893, t76932, t76935, t76936, t76937, t76938, t76939, t76940, t76941);
    (t76932, t76935, t76936, t76937, t76938, t76939, t76940, t76941, t76942, t76943)
}
