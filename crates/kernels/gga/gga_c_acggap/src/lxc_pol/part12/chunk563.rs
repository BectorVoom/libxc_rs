//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 563/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk563<F: Float>(t43: F, t3992: F, t657: F, t2618: F, t2861: F, t474: F, t34: F, t886: F, t234: F, t821: F, t1361: F, t1364: F, t39: F, t47: F, t818: F, t824: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t3993 = t3992 * t657;
    let t3994 = F::cast_from(0.10843581300301739842e-1_f64) * t3993;
    let t3995 = F::cast_from(0.21687162600603479684e-1_f64) * t2618;
    let t3996 = t2861 * t474;
    let t3999 = t886 * t34;
    let t4000 = t821 * t234;
    let t4010 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t3996 * t818 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t3999 * t4000 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1361 * t824 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t47 * t821 - F::cast_from(8.0_f64) * t1364 * t39);
    (t3994, t3995, t4000, t4010)
}
