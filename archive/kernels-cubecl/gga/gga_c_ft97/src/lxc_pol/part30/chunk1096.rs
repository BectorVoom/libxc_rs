//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1096/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1096<F: Float>(t143158: F, t152669: F, t33820: F, t10683: F, t28496: F, t6317: F, t6318: F, t25162: F, t35843: F, t143038: F, t143058: F, t152659: F, t152663: F, t152667: F, t152671: F, t152675: F, t152680: F, t152686: F, t152690: F, t152694: F, t152698: F, t152702: F) -> (F, F, F, F) {
    let t152704 = t33820 * t143158 * t152669;
    let t152708 = t6317 * t10683 * t6318 * t28496;
    let t152710 = t25162 * t35843;
    let t152712 = -t143038 / F::cast_from(54.0_f64) - t152659 / F::cast_from(3.0_f64) - t152663 / F::cast_from(3.0_f64) - t152667 / F::cast_from(36.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152671 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t152675 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t152680 + t143058 / F::cast_from(27.0_f64) - F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t152686 - t152690 / F::cast_from(6.0_f64) - t152694 / F::cast_from(8.0_f64) + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t152698 + t152702 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t152704 - F::cast_from(2.0_f64) * t152708 + t152710 / F::cast_from(27.0_f64);
    (t152704, t152708, t152710, t152712)
}
