//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 777/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk777<F: Float>(t10595: F, t14953: F, t14955: F, t14957: F, t14999: F, t15011: F, t15014: F, t15015: F, t15025: F, t15028: F, t19640: F, t19643: F, t19646: F, t19649: F, t19651: F, t19653: F, t19656: F, t19659: F, t19662: F, t19665: F, t3139: F, t462: F) -> (F,) {
    let t19668 = -t14953 - t14955 + t14957 - t14999 - 8.0 / 9.0 * t15011 + t15014 - 4.0 / 9.0 * t15015 - 8.0 / 27.0 * t15025 - t15028 - 10.0 / 27.0 * t462 * t19640 + 8.0 / 9.0 * t3139 * t19643 + 2.0 / 3.0 * t462 * t19646 - 2.0 / 9.0 * t19649 + t19651 / 3.0 - 2.0 / 3.0 * t19653 - 4.0 / 9.0 * t10595 + t462 * t19656 / 3.0 - 2.0 / 3.0 * t462 * t19659 - 2.0 / 3.0 * t462 * t19662 - 2.0 * t462 * t19665;
    (t19668,)
}
