//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2802/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2802<F: Float>(t1509: F, t5911: F, t105: F, t108: F, t13496: F, t1507: F, t2: F, t21861: F, t21865: F, t21869: F, t21872: F, t21873: F, t2255: F, t22617: F, t22624: F, t2357: F, t4279: F, t4284: F, t46212: F, t49787: F, t580: F, t5902: F, t5907: F, t661: F, t75625: F, t75879: F) -> F {
    let t75906 = t1509 * t5911;
    let t75924 = -F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t5902 * t4284 + F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t1507 * t21861 + F::cast_from(100.0_f64) / F::cast_from(9.0_f64) * t75625 * t21865 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1507 * t21869 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t1507 * t21873 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t105 * t46212 * t22617 * t661 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t49787 * t5907 * t2 * t580 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t49787 * t75906 * t661 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t13496 * t2255 * t5911 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t105 * t4279 * t21872 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t105 * t2357 * t22624 * t661 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t108 * t75879;
    t75924
}
