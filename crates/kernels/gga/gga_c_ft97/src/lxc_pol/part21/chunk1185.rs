//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1185/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1185<F: Float>(t102730: F, t102732: F, t102743: F, t102751: F, t102759: F, t102760: F, t102767: F, t102772: F, t102773: F, t116130: F, t16286: F, t16485: F, t1901: F, t25919: F, t29947: F, t29986: F, t379: F, t39107: F, t446: F, t452: F, t5710: F, t60243: F, t83: F, t8506: F, t91771: F) -> (F,) {
    let t117041 = t102730 + t102732 + 8.0 / 27.0 * t102743 - 4.0 / 9.0 * t1901 * t60243 * t25919 + 2.0 / 9.0 * t1901 * t39107 * t29986 * t379 - 2.0 / 9.0 * t1901 * t91771 * t16485 - t102751 + 2.0 / 3.0 * t446 * t83 * t116130 + 2.0 / 9.0 * t1901 * t8506 * t29947 - t102759 - 8.0 / 81.0 * t102760 - t102767 - t102772 + 8.0 / 81.0 * t102773 + t446 * t452 * t5710 * t16286 / 3.0;
    (t117041,)
}
