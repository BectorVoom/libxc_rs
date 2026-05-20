//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1670/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1670<F: Float>(t52128: F, t63453: F, t63459: F, t63464: F, t63533: F, t63538: F, t63545: F, t77559: F, t77561: F, t77806: F, t77858: F, t88252: F, t88257: F, t88260: F) -> F {
    let t88336 = F::cast_from(0.27785333333333333333e0_f64) * t77806 + F::cast_from(0.12349037037037037037e1_f64) * t52128 - F::cast_from(0.91817777777777777776e0_f64) * t63453 + F::cast_from(0.27545333333333333333e1_f64) * t63459 - F::cast_from(0.23154444444444444445e0_f64) * t63533 + F::cast_from(0.13892666666666666667e1_f64) * t63538 - F::cast_from(0.69463333333333333334e0_f64) * t63545 + F::cast_from(0.13772666666666666667e1_f64) * t77559 - F::new(0.41318e1) * t77561 + F::new(0.6311625e0) * t88252 - F::cast_from(0.13772666666666666666e1_f64) * t63464 + F::cast_from(0.27785333333333333333e0_f64) * t77858 + F::new(0.125034e1) * t88257 - F::new(0.375102e1) * t88260;
    t88336
}
