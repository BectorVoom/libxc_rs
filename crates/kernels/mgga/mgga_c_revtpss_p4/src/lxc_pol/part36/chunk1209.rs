//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1209/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1209<F: Float>(t11006: F, t256: F, t10115: F, t251: F, t2410: F, t11238: F, t196: F, t3800: F, t12625: F, t458: F, t13180: F, t493: F) -> (F, F, F, F, F, F, F) {
    let t41077 = F::cast_from(1.0_f64) / t11006 / t256;
    let t41117 = t10115 * t251;
    let t41153 = t2410 * t2410;
    let t41154 = F::cast_from(1.0_f64) / t41153;
    let t42859 = F::cast_from(1.0_f64) / t11238 / t196;
    let t44125 = t3800 * t3800;
    let t44126 = F::cast_from(1.0_f64) / t44125;
    let t44841 = F::cast_from(1.0_f64) / t12625 / t458;
    let t45551 = F::cast_from(1.0_f64) / t13180 / t493;
    (t41077, t41117, t41154, t42859, t44126, t44841, t45551)
}
