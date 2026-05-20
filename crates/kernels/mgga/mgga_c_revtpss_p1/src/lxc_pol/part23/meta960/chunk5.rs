//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3238/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3238<F: Float>(t13302: F, t13324: F, t1480: F, t18281: F, t21732: F, t21755: F, t21758: F, t22671: F, t22688: F, t22692: F, t2275: F, t2282: F, t4186: F, t4201: F, t4210: F, t4211: F, t44: F, t46065: F, t46074: F, t56: F, t5825: F, t5843: F, t606: F, t614: F) -> F {
    let t85295 = F::new(5.0) / F::new(162.0) * t56 * t46074 * t22688 * t606 + F::new(5.0) / F::new(6.0) * t56 * t13324 * t5825 + F::new(5.0) / F::new(6.0) * t56 * t4210 * t18281 + F::new(5.0) / F::new(18.0) * t56 * t2282 * t22671 * t606 - F::new(5.0) / F::new(36.0) * t44 * t21732 * t4186 + F::new(5.0) / F::new(162.0) * t44 * t46065 * t22688 * t606 + F::new(5.0) / F::new(6.0) * t44 * t13302 * t5825 + F::new(5.0) / F::new(6.0) * t44 * t4201 * t18281 + F::new(5.0) / F::new(18.0) * t44 * t2275 * t22671 * t606 + F::new(220.0) / F::new(27.0) * t5843 * t4211 - F::new(40.0) / F::new(9.0) * t1480 * t21758 - F::new(10.0) / F::new(27.0) * t1480 * t21755 - F::new(20.0) / F::new(9.0) * t614 * t22692;
    t85295
}
