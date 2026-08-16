//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 718/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk718<F: Float>(t409: F, t7685: F, t2100: F, t7676: F, t1988: F, t2092: F, t1459: F, t7458: F, t7486: F, t1980: F, t2117: F, t377: F) -> (F, F, F, F, F, F) {
    let t7686 = t7685 * t409;
    let t7696 = t7676 * t2100;
    let t7697 = F::cast_from(0.18868855373762491241e-2_f64) * t7696;
    let t7698 = t1988 * t2092;
    let t7709 = t7458 * t1459 * t7486;
    let t7710 = t1980 * t7709;
    let t7712 = t377 * t2117;
    (t7686, t7697, t7698, t7709, t7710, t7712)
}
