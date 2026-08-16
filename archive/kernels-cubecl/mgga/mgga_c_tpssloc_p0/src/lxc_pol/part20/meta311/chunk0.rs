//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1566/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1566<F: Float>(t11129: F, t3403: F, t11135: F, t11203: F, t11161: F, t11170: F, t11197: F, t11200: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F) -> (F, F, F, F) {
    let t11366 = t11129 * t3403;
    let t11369 = F::cast_from(0.93932222222222222223e0_f64) * t11135;
    let t11372 = F::cast_from(0.36793333333333333333e0_f64) * t11203;
    let t11383 = -t11369 - F::cast_from(0.3883875e1_f64) * t11197 + F::cast_from(0.247573125e0_f64) * t11200 - t11372 + F::cast_from(0.49671e0_f64) * t11206 + F::cast_from(0.82785e-1_f64) * t11209 + F::cast_from(0.27595e0_f64) * t11211 + F::cast_from(0.5519e-1_f64) * t11213 - F::cast_from(0.33114e0_f64) * t11215 - F::cast_from(0.16557e0_f64) * t11217 + F::cast_from(0.36793333333333333333e-1_f64) * t11221 - F::cast_from(0.16557e0_f64) * t11224 - F::cast_from(0.60384999999999999999e0_f64) * t11161 + F::cast_from(0.181155e1_f64) * t11170;
    (t11366, t11369, t11372, t11383)
}
