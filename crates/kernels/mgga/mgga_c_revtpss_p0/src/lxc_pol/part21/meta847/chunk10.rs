//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3185/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3185<F: Float>(t12571: F, t5207: F, t12486: F, t300: F, t1187: F, t3515: F, t5184: F, t16812: F, t3531: F, t12553: F, t16997: F, t1196: F, t16672: F, t3498: F) -> (F, F, F, F, F, F) {
    let t58664 = F::cast_from(0.51947577317044391277e2_f64) * t12571 * t5207;
    let t58665 = t300 * t12486;
    let t58666 = t3515 * t1187;
    let t58669 = F::cast_from(0.31168546390226634765e3_f64) * t58665 * t5184 * t58666;
    let t58671 = F::cast_from(0.30762056574649219973e4_f64) * t3531 * t16812;
    let t58672 = t300 * t12553;
    let t58675 = F::cast_from(0.30762056574649219974e4_f64) * t58672 * t16997 * t58666;
    let t58678 = F::cast_from(0.10526802520742363173e2_f64) * t1196 * t16672 * t3498;
    (t58664, t58666, t58669, t58671, t58675, t58678)
}
