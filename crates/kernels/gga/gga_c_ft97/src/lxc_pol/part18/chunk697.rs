//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 697/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk697<F: Float>(t3184: F, t8392: F, t10992: F, t10976: F, t10981: F, t10985: F, t10990: F, t10996: F, t11000: F, t11005: F, t11010: F, t11015: F, t8437: F, t11021: F, t11023: F, t11025: F) -> (F, F, F, F, F) {
    let t11913 = 2.0 / 27.0 * t8392 * t3184;
    let t11922 = 2.0 / 9.0 * t10992;
    let t11928 = -t8437 + 4.0 / 9.0 * t10976 + 2.0 / 3.0 * t10981 + t10985 / 3.0 + 2.0 / 9.0 * t10990 - t11922 + 4.0 / 3.0 * t10996 + 2.0 / 3.0 * t11000 + 8.0 / 3.0 * t11005 - 10.0 / 27.0 * t11010 - 8.0 / 9.0 * t11015;
    let t11930 = 2.0 / 9.0 * t11021;
    let t11931 = 4.0 / 9.0 * t11023;
    let t11932 = 4.0 / 27.0 * t11025;
    (t11913, t11928, t11930, t11931, t11932)
}
