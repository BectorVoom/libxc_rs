//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1003/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1003<F: Float>(t24275: F, t9533: F, t684: F, t709: F, t24298: F, t24330: F, t6055: F, t24294: F, t1418: F, t2248: F, t230: F, t1417: F, t22532: F, t3771: F, t6041: F, t1403: F, t1426: F, t9555: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t96716 = t9533 * t24275;
    let t96717 = t684 * t709;
    let t96722 = t24330 * t24298;
    let t96723 = t6055 * t96722;
    let t96725 = t24330 * t24294;
    let t96726 = t6055 * t96725;
    let t96737 = t1418 * t2248 * t230;
    let t96739 = 0.70937342644032921812e-2 * t1417 * t96737;
    let t96750 = t3771 * t6041 * t22532;
    let t96770 = 14.0 / 81.0 * t1403 * t9555 * t1426;
    (t96716, t96717, t96722, t96723, t96725, t96726, t96737, t96739, t96750, t96770)
}
