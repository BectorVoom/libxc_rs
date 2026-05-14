//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 410/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk410<F: Float>(t488: F, t6557: F, t83: F, t1901: F, t28: F, t446: F, t5629: F, t5655: F, t5716: F, t6466: F, t6471: F, t6475: F, t6480: F, t6484: F, t6488: F, t6492: F, t6526: F, t6531: F, t6535: F, t6540: F, t6544: F, t6549: F, t89: F) -> (F, F) {
    let t6558 = t488 * t6557;
    let t6559 = t83 * t6558;
    let t6562 = t5629 + t1901 * t6466 / 9.0 + 2.0 / 3.0 * t446 * t6471 - t446 * t6475 / 3.0 + t446 * t6480 / 3.0 - t446 * t6484 / 3.0 - t5655 - t446 * t6488 / 9.0 - t446 * t6492 / 3.0 + t89 * t28 * t6526 / 3.0 - t446 * t6531 / 3.0 + t5716 + t1901 * t6535 / 9.0 + t446 * t6540 / 3.0 - t446 * t6544 / 3.0 + 2.0 / 3.0 * t446 * t6549 - t446 * t6559 / 3.0;
    (t6559, t6562)
}
