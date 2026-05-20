//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2500/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2500<F: Float>(t12809: F, t12811: F, t12916: F, t12952: F, t3172: F, t3711: F, t12901: F, t13033: F, t13042: F, t13047: F, t3555: F, t3781: F, t5330: F) -> (F, F, F, F, F) {
    let t44637 = t12809 * t12916 * t12811;
    let t44649 = t3711 * t3172 * t12952;
    let t44658 = t13033 * t12901;
    let t44661 = t13042 * t3172 * t13047;
    let t44664 = t3555 * t3781 * t5330;
    (t44637, t44649, t44658, t44661, t44664)
}
