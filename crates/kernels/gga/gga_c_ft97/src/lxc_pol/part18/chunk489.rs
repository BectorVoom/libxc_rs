//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 489/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk489<F: Float>(t488: F, t5743: F, t83: F, t1901: F, t28: F, t446: F, t5629: F, t5632: F, t5637: F, t5641: F, t5646: F, t5650: F, t5655: F, t5657: F, t5661: F, t5706: F, t5712: F, t5716: F, t5719: F, t5724: F, t5728: F, t5733: F, t89: F) -> (F, F, F) {
    let t5744 = t488 * t5743;
    let t5745 = t83 * t5744;
    let t5748 = t5629 + t1901 * t5632 / 9.0 + 2.0 / 3.0 * t446 * t5637 - t446 * t5641 / 3.0 + t446 * t5646 / 3.0 - t446 * t5650 / 3.0 - t5655 - t446 * t5657 / 9.0 - t446 * t5661 / 3.0 + t89 * t28 * t5706 / 3.0 - t446 * t5712 / 3.0 + t5716 + t1901 * t5719 / 9.0 + t446 * t5724 / 3.0 - t446 * t5728 / 3.0 + 2.0 / 3.0 * t446 * t5733 - t446 * t5745 / 3.0;
    (t5744, t5745, t5748)
}
