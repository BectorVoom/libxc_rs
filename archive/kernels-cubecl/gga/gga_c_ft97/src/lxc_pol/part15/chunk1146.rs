//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1146/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1146<F: Float>(t1131: F, t21181: F, t1091: F, t1175: F, t14081: F, t14159: F, t1901: F, t21369: F, t21764: F, t21772: F, t2599: F, t3885: F, t3891: F, t42409: F, t42859: F, t446: F, t4917: F, t5053: F, t68074: F, t724: F, t81170: F, t81183: F, t81207: F, t81209: F, t88098: F) -> (F, F) {
    let t89222 = t21181 * t1131;
    let t89252 = F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t1901 * t42409 * t14081 * t89222 + F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t81170 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t81183 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t446 * t724 * t21772 * t1091 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t446 * t724 * t1175 * t21369 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t68074 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t81207 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t81209 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1901 * t3891 * t42859 * t88098 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t14159 * t21764 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t2599 * t3885 * t4917 * t5053;
    (t89222, t89252)
}
