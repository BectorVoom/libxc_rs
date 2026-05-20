//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2848/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2848<F: Float>(t11341: F, t141: F, t51998: F, t15129: F, t2251: F, t930: F, t41361: F, t41363: F, t41369: F, t51978: F, t51981: F, t51984: F, t51987: F, t51990: F, t51995: F) -> (F, F, F, F) {
    let t52000 = t141 * t11341 * t51998;
    let t52002 = t15129 * t2251;
    let t52004 = t141 * t930 * t52002;
    let t52009 = F::cast_from(0.31310740740740740741e0_f64) * t51978 - F::cast_from(0.8585111111111111111e-1_f64) * t51981 + F::new(0.49671e0) * t51984 + F::new(0.16557e0) * t51987 + F::new(0.49671e0) * t51990 + F::new(0.49671e0) * t51995 + F::new(0.44152e0) * t52000 - F::new(0.149013e1) * t52004 + F::cast_from(0.93932222222222222223e0_f64) * t41361 + F::cast_from(0.80513333333333333335e0_f64) * t41363 - F::cast_from(0.40256666666666666668e0_f64) * t41369;
    (t52000, t52002, t52004, t52009)
}
