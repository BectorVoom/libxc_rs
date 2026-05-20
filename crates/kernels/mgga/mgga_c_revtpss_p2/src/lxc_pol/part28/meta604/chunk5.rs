//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2089/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2089<F: Float>(t2022: F, t9990: F, t1426: F, t786: F, t7911: F, t3917: F, t14230: F, t25924: F, t25926: F, t27837: F, t27868: F, t27973: F, t27980: F, t3999: F, t4077: F, t4131: F, t48020: F, t48074: F, t49393: F, t7274: F, t7295: F, t7296: F, t7910: F, t7920: F, t94593: F, t94598: F, t94602: F, t94605: F, t94656: F, t94705: F) -> F {
    let t97764 = t9990 * t2022;
    let t97783 = t786 * t7911 * t1426;
    let t97785 = F::cast_from(0.19514881078765566038e-1_f64) * t97783 * t3917;
    let t97791 = F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t7910 * t4131 + F::cast_from(0.34270468708064099208e-1_f64) * t94593 - F::cast_from(0.28912093960683998208e-1_f64) * t94598 + F::cast_from(0.10408353825846239354e2_f64) * t7295 * t94656 * t7920 * t4077 + F::cast_from(0.26020884564615598386e1_f64) * t27868 * t97764 * t49393 - F::cast_from(0.26020884564615598386e1_f64) * t27868 * t27980 * t48074 - F::cast_from(0.17347256376410398924e1_f64) * t27868 * t3999 * t7274 * t14230 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t25924 * t7910 * t4077 - F::cast_from(0.17347256376410398924e1_f64) * t27868 * t27980 * t48020 - t97785 - F::cast_from(0.26020884564615598386e1_f64) * t27837 * t25926 + t94602 - F::cast_from(0.14456046980341999104e-1_f64) * t94605 - F::cast_from(0.17347256376410398924e1_f64) * t94705 * t27973;
    t97791
}
