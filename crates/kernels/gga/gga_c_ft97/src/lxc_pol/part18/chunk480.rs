//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 480/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk480<F: Float>(t110: F, t1871: F, t5635: F, t1307: F, t452: F, t499: F, t492: F, t488: F, t5617: F, t1328: F, t376: F, t89: F, t1339: F, t379: F, t447: F, t432: F) -> (F, F, F, F, F, F, F, F) {
    let t5637 = t1871 * t110 * t5635;
    let t5641 = t452 * t499 * t1307;
    let t5644 = t1307 * t492;
    let t5646 = t452 * t488 * t5644;
    let t5650 = t452 * t110 * t5617;
    let t5655 = t89 * t376 * t1328 / 9.0;
    let t5657 = t447 * t1339 * t379;
    let t5661 = t452 * t1339 * t432;
    (t5637, t5641, t5644, t5646, t5650, t5655, t5657, t5661)
}
