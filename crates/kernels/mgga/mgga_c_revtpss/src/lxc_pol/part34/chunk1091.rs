//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1091/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1091<F: Float>(t112: F, t843: F, t239: F, t655: F, t2339: F, t624: F, t10208: F, t68: F, t1892: F, t786: F, t25877: F, t1426: F, t7911: F, t2435: F, t27986: F, t1904: F, t2439: F, t25916: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94973 = t843 * t112;
    let t94974 = 154.0 / 27.0 * t94973;
    let t94975 = t239 * t655;
    let t94978 = t624 * t2339;
    let t94982 = t68 * t10208;
    let t97699 = t786 * t1892;
    let t97700 = t97699 * t25877;
    let t97783 = t786 * t7911 * t1426;
    let t97792 = t2435 * t27986;
    let t97795 = t2439 * t25916 * t1904;
    (t94974, t94975, t94978, t94982, t97699, t97700, t97783, t97792, t97795)
}
