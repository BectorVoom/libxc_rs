//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 821/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk821<F: Float>(t1751: F, t5546: F, t37: F, t401: F, t78: F, t51: F, t58: F, t5544: F, t22825: F, t388: F, t1675: F, t444: F, t1302: F, t5603: F, t5607: F, t1300: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22829 = t5546 * t1751;
    let t22833 = t37 * t401;
    let t22834 = t22833 * t78;
    let t22837 = t51 * sigma0;
    let t22838 = t22837 * t58;
    let t22839 = t5544 * t22838;
    let t22842 = t388 * t22825;
    let t22845 = t444 * t1675;
    let t22846 = t22845 * t1302;
    let t22849 = t5603 * t5607;
    let t22850 = t1300 * t22849;
    (t22829, t22833, t22834, t22839, t22842, t22845, t22846, t22849, t22850)
}
