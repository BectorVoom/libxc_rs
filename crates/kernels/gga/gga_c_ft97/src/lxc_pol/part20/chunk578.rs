//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 578/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk578<F: Float>(t10397: F, t1882: F, t2846: F, t2253: F, t2953: F, t170: F, t328: F, t8715: F, t8640: F, t906: F, t2925: F, t8675: F, t2930: F, t703: F, t900: F, t230: F, t2938: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10797 = 28.0 / 27.0 * t10397;
    let t10804 = t1882 * t2846;
    let t10835 = t2253 * t2953;
    let t10838 = 20.0 / 27.0 * t170 * t8715 * t328;
    let t10839 = t8640 * t906;
    let t10841 = t8675 * t2925;
    let t10843 = t2253 * t2930;
    let t10845 = t703 * t900;
    let t10864 = t230 * t2938;
    (t10797, t10804, t10835, t10838, t10839, t10841, t10843, t10845, t10864)
}
