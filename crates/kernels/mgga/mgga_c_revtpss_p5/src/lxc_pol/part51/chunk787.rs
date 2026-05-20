//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 787/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk787<F: Float>(t25253: F, t1945: F, t2693: F, t807: F, t2718: F, t64: F, t7036: F, t820: F, t843: F, t839: F, t241: F, t159: F, t2698: F) -> (F, F, F, F, F, F, F) {
    let t25254 = F::cast_from(0.15244095330869239812e-3_f64) * t25253;
    let t25255 = t1945 * t2693;
    let t25256 = t807 * t25255;
    let t25260 = t2718 * t64;
    let t25266 = t820 * t7036 * t843;
    let t25267 = t25266 * t839;
    let t25270 = t820 * t7036 * t241;
    let t25273 = t2698 * t159;
    (t25254, t25256, t25260, t25266, t25267, t25270, t25273)
}
