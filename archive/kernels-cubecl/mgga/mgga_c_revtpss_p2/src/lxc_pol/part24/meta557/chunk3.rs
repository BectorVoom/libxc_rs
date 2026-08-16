//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1668/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1668<F: Float>(t41690: F, t51978: F, t77736: F, t88118: F, t88126: F, t88134: F, t88168: F, t88171: F, t88203: F, t88206: F, t88209: F, t88211: F, t88214: F, t88216: F) -> F {
    let t88305 = F::cast_from(0.250068e1_f64) * t88168 + F::cast_from(0.62517e0_f64) * t88171 + t41690 + F::cast_from(0.166712e1_f64) * t77736 + F::cast_from(0.21424148148148148148e1_f64) * t51978 - F::cast_from(0.34431666666666666667e1_f64) * t88118 + F::cast_from(0.123954e2_f64) * t88126 - F::cast_from(0.13772666666666666667e1_f64) * t88134 + F::cast_from(0.3529725e1_f64) * t88203 - F::cast_from(0.6618234375e1_f64) * t88206 - F::cast_from(0.13892666666666666667e0_f64) * t88209 + F::cast_from(0.2366859375e0_f64) * t88211 + F::cast_from(0.94674375e0_f64) * t88214 - F::cast_from(0.52945875e1_f64) * t88216;
    t88305
}
