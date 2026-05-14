//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 781/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk781<F: Float>(t275: F, t9598: F, t1347: F, t2479: F, t1562: F, t8048: F, t2474: F, t934: F, t41579: F, t41581: F, t41585: F, t41604: F, t41613: F, t41619: F, t41654: F, t41656: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43654 = 2.0 * t275 * t9598;
    let t43680 = t1347 * t2479;
    let t43722 = 0.4726e1 * t1562 * t8048;
    let t43723 = t934 * t2474;
    let t43745 = 0.1489760996265424379e-3 * t41579;
    let t43746 = 0.39726959900411316772e-4 * t41581;
    let t43752 = 0.11918087970123395032e-3 * t41585;
    let t43757 = 0.60975299583150056624e-3 * t41604;
    let t43761 = 0.60975299583150056624e-3 * t41613;
    let t43763 = 0.60975299583150056624e-3 * t41619;
    let t43783 = 0.11918087970123395032e-3 * t41654;
    let t43784 = 0.36366215538993788974e-1 * t41656;
    (t43654, t43680, t43722, t43723, t43745, t43746, t43752, t43757, t43761, t43763, t43783, t43784)
}
