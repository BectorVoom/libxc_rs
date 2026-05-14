//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 860/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk860<F: Float>(t15446: F, t2881: F, t10443: F, t4256: F, t10730: F, t10732: F, t11593: F, t15404: F, t15409: F, t15415: F, t15419: F, t15420: F, t15422: F, t15427: F, t15430: F, t15435: F, t15438: F, t15443: F, t1901: F, t446: F) -> (F,) {
    let t15447 = t2881 * t15446;
    let t15450 = t10443 * t4256;
    let t15453 = 4.0 / 9.0 * t1901 * t15404 - 4.0 / 9.0 * t11593 * t15409 - 2.0 / 9.0 * t10730 - 8.0 / 81.0 * t10732 - 2.0 / 3.0 * t446 * t15415 + t15419 - 4.0 / 27.0 * t15420 - t446 * t15422 / 3.0 + 2.0 / 3.0 * t446 * t15427 + 2.0 / 3.0 * t446 * t15430 + t446 * t15435 / 3.0 + 4.0 / 3.0 * t446 * t15438 - 4.0 / 9.0 * t11593 * t15443 - 8.0 / 9.0 * t11593 * t15447 + 2.0 / 9.0 * t1901 * t15450;
    (t15453,)
}
