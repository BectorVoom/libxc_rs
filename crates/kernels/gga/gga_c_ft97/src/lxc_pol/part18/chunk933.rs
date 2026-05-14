//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 933/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk933<F: Float>(t22914: F, t6418: F, t3266: F, t5502: F, t8411: F, t1286: F, t22935: F, t25525: F, t25530: F, t25535: F, t25539: F, t25543: F, t25546: F, t25553: F, t25558: F, t5495: F, t5501: F, t5504: F, t5620: F, t5624: F, t6414: F, t6461: F) -> (F, F) {
    let t25561 = t22914 * t6418;
    let t25564 = t8411 * t5502 * t3266;
    let t25568 = -t1286 * t25525 / 3.0 - t1286 * t25530 / 3.0 - t1286 * t25535 / 3.0 + t1286 * t25539 / 6.0 - t25543 / 18.0 - t25546 / 18.0 + t6414 * t5620 / 6.0 + t6414 * t5624 / 6.0 + t1286 * t25553 / 6.0 + t5495 * t6461 / 6.0 - t25558 * t5504 / 18.0 + t25561 / 54.0 + t5501 * t25564 - t22935 * t6418 / 18.0;
    (t25564, t25568)
}
