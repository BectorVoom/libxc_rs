//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2584/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2584<F: Float>(t3906: F, t3907: F, t39494: F, t1426: F, t4067: F, t786: F, t3917: F, t2453: F, t3908: F, t10115: F, t1421: F, t10168: F, t3920: F) -> (F, F, F, F, F, F) {
    let t47504 = F::cast_from(0.20561456923286030469e-1_f64) * t3906 * t3907 * t39494;
    let t47506 = t786 * t4067 * t1426;
    let t47507 = t47506 * t3917;
    let t47510 = t2453 * t4067 * t3908;
    let t47512 = t10115 * t1421;
    let t47516 = t10168 * t3920;
    (t47504, t47506, t47507, t47510, t47512, t47516)
}
