//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 762/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk762<F: Float>(t191: F, t5263: F, t4939: F, t1740: F, t579: F, t1867: F, t582: F, t185: F, t1660: F, t9: F, t1665: F, t587: F) -> (F, F, F, F, F, F) {
    let t5264 = t191 * t5263;
    let t5271 = F::new(0.11197407407407407407e0) * t4939;
    let t5278 = t579 * t1740;
    let t5280 = t582 * t1867;
    let t5281 = t185 * t5280;
    let t5283 = t9 * t1660;
    let t5284 = t5283 * t1665;
    let t5285 = t587 * t5284;
    (t5264, t5271, t5278, t5281, t5283, t5285)
}
