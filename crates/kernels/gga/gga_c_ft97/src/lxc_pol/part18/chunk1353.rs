//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1353/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1353<F: Float>(t105590: F, t105596: F, t105598: F, t105603: F, t105608: F, t105611: F, t105614: F, t105617: F, t105620: F, t105623: F, t95151: F, t95154: F, t105637: F, t105629: F, t105633: F, t105641: F, t105645: F, t105649: F, t105653: F, t105657: F, t105660: F, t105665: F, t95177: F, t95187: F) -> (F, F) {
    let t106045 = 2.0 / 9.0 * t105590 + 5.0 / 16.0 * t105596 + 2.0 / 81.0 * t105598 - t95151 / 36.0 - t95154 / 18.0 + 2.0 / 3.0 * t105603 + t105608 / 6.0 - 4.0 / 27.0 * t105611 - 4.0 / 9.0 * t105614 + 4.0 / 27.0 * t105617 - 2.0 / 9.0 * t105620 - 4.0 / 9.0 * t105623;
    let t106049 = 4.0 / 9.0 * t105637;
    let t106058 = t105629 / 6.0 - t105633 / 18.0 + 16.0 / 27.0 * t95177 - t106049 + 2.0 / 3.0 * t105641 + 2.0 / 3.0 * t105645 - 2.0 * t105649 - t105653 / 54.0 + 2.0 / 27.0 * t95187 - 4.0 * t105657 + 2.0 / 3.0 * t105660 + t105665 / 3.0;
    (t106045, t106058)
}
