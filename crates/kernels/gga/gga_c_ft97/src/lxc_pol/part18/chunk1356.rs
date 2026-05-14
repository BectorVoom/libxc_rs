//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1356/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1356<F: Float>(t105809: F, t105815: F, t105826: F, t105813: F, t105819: F, t105824: F, t105830: F, t105834: F, t105838: F, t95269: F, t95289: F, t105846: F, t105848: F, t105862: F, t105843: F, t105853: F, t105856: F, t105860: F, t105866: F, t105871: F, t105876: F, t95301: F, t95304: F) -> (F, F) {
    let t106115 = t105809 / 54.0;
    let t106118 = t105815 / 9.0;
    let t106121 = 2.0 / 27.0 * t105826;
    let t106125 = 4.0 / 27.0 * t95269 + t106115 - t105813 / 3.0 + t95289 / 9.0 + t106118 + t105819 / 9.0 + 2.0 / 27.0 * t105824 - t106121 + t105830 / 9.0 + 4.0 / 3.0 * t105834 + 4.0 / 3.0 * t105838;
    let t106127 = t105846 / 12.0;
    let t106128 = t105848 / 9.0;
    let t106133 = 4.0 / 3.0 * t105862;
    let t106138 = 2.0 / 3.0 * t105843 + t106127 + t106128 + 8.0 / 81.0 * t95301 - 2.0 * t105853 - 2.0 / 27.0 * t105856 + t105860 / 9.0 + t106133 + 8.0 * t105866 - t105871 / 4.0 - t105876 / 8.0 + t95304 / 18.0;
    (t106125, t106138)
}
