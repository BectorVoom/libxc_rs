//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2222/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2222<F: Float>(t28150: F, t8143: F, t108978: F, t2122: F, t108986: F, t101230: F, t104203: F, t104208: F, t104314: F, t104332: F, t108966: F, t108975: F, t108983: F, t108990: F, t25162: F, t26792: F, t26795: F, t28147: F, t28154: F, t29380: F) -> F {
    let t111665 = t8143 * t28150;
    let t111670 = t2122 * t108978;
    let t111675 = t2122 * t108986;
    let t111680 = -F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t104332 - F::cast_from(10.0_f64) * t104208 * t28147 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t104314 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101230 * t29380 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t108966 * t26795 - F::cast_from(10.0_f64) * t104203 * t28147 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t111665 - F::cast_from(10.0_f64) * t26792 * t108975 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t111670 - F::cast_from(5.0_f64) * t26792 * t108983 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t25162 * t111675 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t108990 * t26795;
    t111680
}
