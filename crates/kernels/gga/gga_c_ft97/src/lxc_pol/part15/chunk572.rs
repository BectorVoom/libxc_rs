//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 572/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk572<F: Float>(t10397: F, t192: F, t7640: F, t869: F, t309: F, t2770: F, t871: F, t313: F, t89: F, t9555: F, t295: F, t9568: F, t170: F, t328: F, t8715: F, t703: F, t900: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10658 = 28.0 / 81.0 * t10397;
    let t10683 = t192 * t7640;
    let t10695 = t869 * t869;
    let t10696 = 1.0 / t10695;
    let t10697 = t309 * t10696;
    let t10703 = t2770 * t871;
    let t10749 = 28.0 / 81.0 * t89 * t9555 * t313;
    let t10758 = t9568 * t295;
    let t10797 = 28.0 / 27.0 * t10397;
    let t10838 = 20.0 / 27.0 * t170 * t8715 * t328;
    let t10845 = t703 * t900;
    (t10658, t10683, t10695, t10696, t10697, t10703, t10749, t10758, t10797, t10838, t10845)
}
