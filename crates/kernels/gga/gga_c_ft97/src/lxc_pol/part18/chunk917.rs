//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 917/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk917<F: Float>(t23938: F, t605: F, t144: F, t1882: F, t5886: F, t5866: F, t5875: F, t1366: F, t8232: F, t1901: F, t23556: F, t23560: F, t23564: F, t23568: F, t23573: F, t23576: F, t23578: F, t23583: F, t23587: F, t23591: F, t23595: F, t23598: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t23939 = t605 * t23938;
    let t23940 = t144 * t23939;
    let t23943 = t1882 * t5886;
    let t23945 = t1882 * t5866;
    let t23947 = t1882 * t5875;
    let t23950 = 4.0 / 27.0 * t8232 * t1366;
    let t23951 = -4.0 / 9.0 * t1901 * t23556 - 2.0 / 9.0 * t1901 * t23560 + t1901 * t23564 / 9.0 + 2.0 / 27.0 * t1901 * t23568 - 4.0 / 3.0 * t1901 * t23573 - 2.0 / 27.0 * t23576 + 2.0 / 9.0 * t1901 * t23578 + 2.0 / 9.0 * t1901 * t23583 + 2.0 / 3.0 * t446 * t23587 - t446 * t23591 / 9.0 - 2.0 / 27.0 * t446 * t23595 + 2.0 / 27.0 * t23598 - t446 * t23940 / 3.0 + 2.0 / 9.0 * t23943 + 2.0 / 9.0 * t23945 + 2.0 / 9.0 * t23947 - t23950;
    (t23939, t23940, t23943, t23945, t23947, t23950, t23951)
}
