//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 555/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk555<F: Float>(t3427: F, t64: F, t2919: F, t871: F, t9664: F, t9666: F, t9674: F, t9676: F, t10627: F, t688: F, t779: F, t2508: F) -> (F, F, F, F, F, F, F) {
    let t10660 = F::new(4.0) / F::new(3.0) * t3427 * t64;
    let t10661 = t2919 * t871;
    let t10663 = F::new(7.0) / F::new(256.0) * t9664;
    let t10664 = F::new(21.0) / F::new(8192.0) * t9666;
    let t10665 = F::new(7.0) / F::new(8192.0) * t9674;
    let t10666 = F::new(7.0) / F::new(768.0) * t9676;
    let t10682 = t10627 * t688;
    let t10683 = t779 * t10682;
    let t10685 = F::new(0.76905262301422242837e-2) * t2508 * t10683;
    (t10660, t10661, t10663, t10664, t10665, t10666, t10685)
}
