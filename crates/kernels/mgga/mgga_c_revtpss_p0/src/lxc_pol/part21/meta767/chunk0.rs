//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2719/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2719<F: Float>(t50014: F, t50033: F, t162: F, t187: F, t40092: F, t40094: F, t14365: F, t14397: F, t2403: F, t39818: F, t39823: F, t40084: F, t40088: F, t49992: F, t49994: F, t49995: F) -> (F, F, F, F, F) {
    let t50034 = t50014 + t50033;
    let t50037 = F::cast_from(0.19751673498613801407e-1_f64) * t50034 * t162 * t187;
    let t50038 = F::cast_from(0.15584273195113317383e3_f64) * t40092;
    let t50039 = F::cast_from(0.10526802520742363173e2_f64) * t40094;
    let t50040 = -F::new(18.0) * t14365 * t14397 * t2403 - t39818 - t39823 + t40084 + t40088 + t49992 + t49994 - t49995 + t50037 - t50038 + t50039;
    (t50034, t50037, t50038, t50039, t50040)
}
