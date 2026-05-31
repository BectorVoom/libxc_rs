//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1154/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1154<F: Float>(t89497: F, t89513: F, t89529: F, t89545: F, t21639: F, t3977: F, t10007: F, t1091: F, t1175: F, t14175: F, t1901: F, t193: F, t21355: F, t21399: F, t21499: F, t241: F, t242: F, t2568: F, t2574: F, t258: F, t2594: F, t42928: F, t446: F, t4934: F, t4969: F, t5064: F, t5073: F, t729: F, t81697: F, t81721: F, t81723: F, t89: F) -> (F, F, F) {
    let t89547 = t89497 + t89513 + t89529 + t89545;
    let t89565 = t3977 * t21639;
    let t89573 = t42928 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t1175 * t21399 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t14175 * t21499 * t1091 + t89 * t193 * t241 * t89547 * t258 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t81697 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t81721 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t81723 + F::cast_from(8.0_f64) * t446 * t2574 * t2568 * t4934 * t5064 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t446 * t2594 * t1175 * t21355 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t89565 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t10007 * t4969 * t5073;
    (t89547, t89565, t89573)
}
