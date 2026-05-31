//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1568/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1568<F: Float>(t3478: F, t434: F, t1175: F, t3520: F, t3519: F, t444: F) -> (F, F, F) {
    let t12472 = F::cast_from(1.0_f64) / t3478 / t434;
    let t12481 = t1175 * t3520;
    let t12485 = F::cast_from(1.0_f64) / t3519 / t444;
    (t12472, t12481, t12485)
}
