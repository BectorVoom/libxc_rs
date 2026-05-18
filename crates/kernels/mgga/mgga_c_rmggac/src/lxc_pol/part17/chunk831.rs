//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 831/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk831<F: Float>(t40965: F, t7835: F, t39666: F, t7788: F, t262: F, t40805: F, t7782: F, t1587: F, t664: F, t2067: F, t26: F, t25525: F) -> (F, F, F, F, F, F, F) {
    let t40966 = t7835 * t40965;
    let t40967 = F::new(0.36366215538993788972e-1) * t40966;
    let t40970 = t7788 * t39666;
    let t40975 = t262 * t40805;
    let t40976 = t7782 * t40975;
    let t40983 = t664 * t1587;
    let t40998 = t2067 * t26;
    let t40999 = t25525 * t40998;
    (t40967, t40970, t40975, t40976, t40983, t40998, t40999)
}
