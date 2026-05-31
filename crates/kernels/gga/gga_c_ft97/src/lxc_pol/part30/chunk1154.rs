//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1154/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1154<F: Float>(t2749: F, t36042: F, t36145: F, t8392: F, t143038: F, t143058: F, t152659: F, t152663: F, t152667: F, t152671: F, t152675: F, t152680: F, t152686: F, t152690: F, t152694: F, t152698: F, t152702: F, t152704: F, t152708: F, t152710: F) -> (F, F, F) {
    let t154083 = t2749 * t36042;
    let t154090 = t8392 * t36145;
    let t154111 = -t143038 / F::cast_from(18.0_f64) - t152659 - t152663 - t152667 / F::cast_from(12.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t152671 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t152675 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152680 + t143058 / F::cast_from(9.0_f64) - F::cast_from(15.0_f64) / F::cast_from(4.0_f64) * t152686 - t152690 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t152694 + F::cast_from(8.0_f64) * t152698 + F::cast_from(3.0_f64) * t152702 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t152704 - F::cast_from(6.0_f64) * t152708 + t152710 / F::cast_from(9.0_f64);
    (t154083, t154090, t154111)
}
