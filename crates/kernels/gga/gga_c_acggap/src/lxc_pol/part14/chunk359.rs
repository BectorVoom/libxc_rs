//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 359/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk359<F: Float>(t43: F, t50: F, t1690: F, t1694: F, t47: F, t886: F, t478: F, t52: F, t893: F, t59: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1698 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t886 * t1690 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t1694);
    let t1699 = t478 * t478;
    let t1702 = -t1694;
    let t1706 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t893 * t1699 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t1702);
    let t1708 = (t1698 + t1706) * t59;
    (t1699, t1702, t1708)
}
