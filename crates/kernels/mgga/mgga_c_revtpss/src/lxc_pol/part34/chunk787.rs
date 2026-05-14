//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 787/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk787<F: Float>(t12552: F, t439: F, t3522: F, t447: F, t3800: F, t498: F, t12295: F, t1207: F, t456: F, t487: F, t3566: F, t3754: F, t1209: F, t5462: F, t5477: F, t3634: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12553 = t439 * t12552;
    let t12555 = 1.0 / t3522 / t447;
    let t12587 = 1.0 / t3800 / t498;
    let t12610 = 0.46096296296296296297e-1 * t12295;
    let t12625 = t1207 * t1207;
    let t12626 = 1.0 / t12625;
    let t12627 = t456 * t12626;
    let t12628 = t12627 * t487;
    let t12678 = 0.25925925925925925926e-1 * t12295;
    let t12717 = t3566 * t3754;
    let t12751 = t1209 * t5462;
    let t12756 = t1209 * t5477;
    let t12772 = t828 * t3634;
    (t12553, t12555, t12587, t12610, t12627, t12628, t12678, t12717, t12751, t12756, t12772)
}
