//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1598/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1598<F: Float>(t1480: F, t1483: F, t21732: F, t21754: F, t22671: F, t22700: F, t22709: F, t22712: F, t22715: F, t2275: F, t2282: F, t4201: F, t4210: F, t44: F, t46065: F, t46074: F, t46090: F, t48: F, t56: F, t5825: F, t5843: F, t5848: F, t5851: F, t60: F, t61: F, t87107: F, t87126: F, t87132: F, t87145: F) -> F {
    let t87155 = -F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t44 * t21732 * t5825 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t2275 * t87107 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t44 * t4201 * t22671 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t1480 * t22712 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t56 * t21754 * t5825 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t2282 * t87107 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t56 * t4210 * t22671 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t48 * t87126 + F::cast_from(20944.0_f64) / F::cast_from(81.0_f64) * t87132 * t61 + F::cast_from(12320.0_f64) / F::cast_from(81.0_f64) * t22700 * t1483 - F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t5843 * t5851 + F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t5843 * t5848 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1480 * t22709 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t1480 * t22715 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t56 * t46074 * t87145 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t60 * t87126 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t44 * t46065 * t87145 - t46090;
    t87155
}
