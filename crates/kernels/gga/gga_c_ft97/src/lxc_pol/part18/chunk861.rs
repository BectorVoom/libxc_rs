//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 861/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk861<F: Float>(t1901: F, t23263: F, t23267: F, t23271: F, t23274: F, t23277: F, t23280: F, t23283: F, t23285: F, t23288: F, t23291: F, t23296: F, t23300: F, t23304: F, t23307: F, t23311: F, t23312: F, t23315: F, t446: F) -> (F,) {
    let t23318 = -4.0 / 9.0 * t23263 - 2.0 / 9.0 * t1901 * t23267 - 4.0 / 9.0 * t1901 * t23271 - t446 * t23274 / 3.0 - 2.0 / 3.0 * t446 * t23277 - t446 * t23280 / 3.0 + 2.0 / 9.0 * t23283 + 2.0 / 3.0 * t446 * t23285 - 2.0 / 3.0 * t446 * t23288 - 2.0 * t446 * t23291 + 2.0 / 9.0 * t1901 * t23296 - 2.0 / 9.0 * t1901 * t23300 - 2.0 / 9.0 * t446 * t23304 - t446 * t23307 / 3.0 - t23311 + 2.0 / 9.0 * t23312 - t446 * t23315 / 3.0;
    (t23318,)
}
