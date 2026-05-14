//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 606/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk606<F: Float>(t8232: F, t837: F, t877: F, t313: F, t89: F, t9555: F, t295: F, t9568: F, t842: F, t10397: F, t170: F, t328: F, t8715: F, t8640: F, t906: F, t703: F, t900: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10732 = t8232 * t837;
    let t10735 = t8232 * t877;
    let t10749 = 28.0 / 81.0 * t89 * t9555 * t313;
    let t10758 = t9568 * t295;
    let t10773 = t8232 * t842;
    let t10797 = 28.0 / 27.0 * t10397;
    let t10838 = 20.0 / 27.0 * t170 * t8715 * t328;
    let t10839 = t8640 * t906;
    let t10845 = t703 * t900;
    (t10732, t10735, t10749, t10758, t10773, t10797, t10838, t10839, t10845)
}
