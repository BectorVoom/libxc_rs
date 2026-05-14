//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 971/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk971<F: Float>(t25923: F, t25952: F, t25995: F, t26039: F, t103: F, t6547: F, t8466: F, t22943: F, t3219: F, t11837: F, t1332: F, t1825: F, t6557: F, t22868: F, t22872: F, t22879: F, t22893: F, t25856: F, t25863: F, t25867: F, t25869: F, t5495: F, t5501: F, t5510: F, t6414: F, t6457: F, t8418: F) -> (F, F, F, F, F, F, F) {
    let t26041 = t25923 + t25952 + t25995 + t26039;
    let t26042 = t26041 * t103;
    let t26045 = t8466 * t6547;
    let t26048 = t22943 * t3219;
    let t26050 = t11837 * t1332;
    let t26052 = t1825 * t6557;
    let t26054 = t5495 * t6457 / 6.0 - 12.0 * t8418 * t25856 - t6414 * t5510 / 3.0 - t5501 * t25863 / 18.0 - t22868 / 18.0 + t22872 - 2.0 * t25867 - t25869 / 18.0 + 2.0 * t26042 + t22879 / 9.0 + 4.0 * t26045 - t22893 / 18.0 + 4.0 * t26048 - 2.0 * t26050 - 2.0 * t26052;
    (t26041, t26042, t26045, t26048, t26050, t26052, t26054)
}
