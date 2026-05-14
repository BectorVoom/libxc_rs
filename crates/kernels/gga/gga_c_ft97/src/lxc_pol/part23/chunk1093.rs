//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1093/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1093<F: Float>(t2432: F, t70: F, t6034: F, t6037: F, t2383: F, t2427: F, t27671: F, t33374: F, t24275: F, t9533: F, t1418: F, t2248: F, t230: F, t1417: F, t22532: F, t3771: F, t6041: F) -> (F, F, F, F, F, F, F, F) {
    let t96690 = t2432 * t70;
    let t96692 = t6034 * t96690 * t6037;
    let t96694 = t2383 * t2427;
    let t96696 = t27671 * t33374;
    let t96716 = t9533 * t24275;
    let t96737 = t1418 * t2248 * t230;
    let t96739 = 0.70937342644032921812e-2 * t1417 * t96737;
    let t96750 = t3771 * t6041 * t22532;
    (t96690, t96692, t96694, t96696, t96716, t96737, t96739, t96750)
}
