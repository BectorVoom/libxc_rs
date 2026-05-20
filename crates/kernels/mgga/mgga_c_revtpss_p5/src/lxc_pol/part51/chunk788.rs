//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 788/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk788<F: Float>(t218: F, t25273: F, t816: F, t228: F, t7021: F, t802: F, t7043: F, t826: F, t2736: F, t7082: F, t72: F, t686: F) -> (F, F, F, F, F) {
    let t25275 = t25273 * t218 * t816;
    let t25276 = F::new(35.0) / F::new(432.0) * t25275;
    let t25277 = t7021 * t228;
    let t25278 = t25277 * t802;
    let t25282 = t7043 * t826;
    let t25283 = t2736 * t25282;
    let t25284 = F::cast_from(0.50820002809285328225e-5_f64) * t25283;
    let t25295 = t7082 * t72;
    let t25296 = t25295 * t686;
    (t25276, t25277, t25278, t25284, t25296)
}
