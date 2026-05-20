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
    let t75924 = -F::new(200.0) / F::new(9.0) * t5902 * t4284 + F::new(50.0) / F::new(27.0) * t1507 * t21861 + F::new(100.0) / F::new(9.0) * t75625 * t21865 - F::new(50.0) / F::new(9.0) * t1507 * t21869 - F::new(25.0) / F::new(3.0) * t1507 * t21873 + F::new(40.0) / F::new(81.0) * t105 * t46212 * t22617 * t661 + F::new(10.0) / F::new(9.0) * t49787 * t5907 * t2 * t580 - F::new(10.0) / F::new(9.0) * t49787 * t75906 * t661 - F::new(10.0) / F::new(3.0) * t13496 * t2255 * t5911 + F::new(10.0) / F::new(3.0) * t105 * t4279 * t21872 + F::new(10.0) / F::new(9.0) * t105 * t2357 * t22624 * t661 - F::new(5.0) / F::new(3.0) * t105 * t108 * t75879;
    t75924
}
