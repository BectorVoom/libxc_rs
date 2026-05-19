//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 857/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk857<F: Float>(t12295: F, t3566: F, t3754: F, t1209: F, t5462: F, t5477: F, t3634: F, t828: F, t3618: F, t3781: F, t5330: F, t1121: F, t3603: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12678 = F::cast_from(0.25925925925925925926e-1_f64) * t12295;
    let t12717 = t3566 * t3754;
    let t12751 = t1209 * t5462;
    let t12756 = t1209 * t5477;
    let t12772 = t828 * t3634;
    let t12787 = t828 * t3618;
    let t12808 = t1209 * t3781;
    let t12809 = t12808 * t5330;
    let t12839 = t3603 * t1121;
    (t12678, t12717, t12751, t12756, t12772, t12787, t12808, t12809, t12839)
}
