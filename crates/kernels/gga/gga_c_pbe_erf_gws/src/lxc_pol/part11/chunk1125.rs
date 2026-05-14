//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1125/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1125<F: Float>(t50110: F, t50115: F, t50116: F, t50128: F, t50135: F, t50137: F, t50146: F, t50158: F, t50160: F, t50162: F, t50168: F, t50187: F, t50189: F, t50193: F, t50201: F, t50206: F, t50207: F, t50212: F, t50219: F, t50220: F, t50230: F, t50231: F) -> (F, F) {
    let t50586 = t50110 - t50115 + t50116 - t50128 + t50135 + t50137 - t50146 + t50158 - t50160 - t50162 - t50168;
    let t50587 = -t50187 + t50189 + t50193 - t50201 - t50206 - t50207 - t50212 - t50219 - t50220 - t50230 + t50231;
    (t50586, t50587)
}
