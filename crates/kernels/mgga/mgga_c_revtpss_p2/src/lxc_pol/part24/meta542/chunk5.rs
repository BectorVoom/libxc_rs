//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1598/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1598<F: Float>(t1480: F, t1483: F, t21732: F, t21754: F, t22671: F, t22700: F, t22709: F, t22712: F, t22715: F, t2275: F, t2282: F, t4201: F, t4210: F, t44: F, t46065: F, t46074: F, t46090: F, t48: F, t56: F, t5825: F, t5843: F, t5848: F, t5851: F, t60: F, t61: F, t87107: F, t87126: F, t87132: F, t87145: F) -> F {
    let t87155 = -F::new(5.0) / F::new(18.0) * t44 * t21732 * t5825 + F::new(5.0) / F::new(6.0) * t44 * t2275 * t87107 + F::new(10.0) / F::new(9.0) * t44 * t4201 * t22671 - F::new(80.0) / F::new(9.0) * t1480 * t22712 + F::new(5.0) / F::new(18.0) * t56 * t21754 * t5825 + F::new(5.0) / F::new(6.0) * t56 * t2282 * t87107 + F::new(10.0) / F::new(9.0) * t56 * t4210 * t22671 + F::new(5.0) / F::new(6.0) * t44 * t48 * t87126 + F::new(20944.0) / F::new(81.0) * t87132 * t61 + F::new(12320.0) / F::new(81.0) * t22700 * t1483 - F::new(440.0) / F::new(9.0) * t5843 * t5851 + F::new(440.0) / F::new(27.0) * t5843 * t5848 - F::new(40.0) / F::new(81.0) * t1480 * t22709 + F::new(80.0) / F::new(9.0) * t1480 * t22715 + F::new(5.0) / F::new(162.0) * t56 * t46074 * t87145 - F::new(5.0) / F::new(6.0) * t56 * t60 * t87126 + F::new(5.0) / F::new(162.0) * t44 * t46065 * t87145 - t46090;
    t87155
}
