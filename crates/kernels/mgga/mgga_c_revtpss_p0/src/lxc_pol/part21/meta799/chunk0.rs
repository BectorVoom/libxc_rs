//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2893/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2893<F: Float>(t15494: F, t964: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51878: F, t51881: F, t51884: F, t51887: F) -> (F, F) {
    let t52522 = t15494 * t964;
    let t52536 = F::new(0.123954e2) * t51849 - F::cast_from(0.34431666666666666667e0_f64) * t51853 - F::cast_from(0.15302962962962962963e1_f64) * t51858 + F::new(0.309885e1) * t51863 + F::new(0.309885e1) * t51867 + F::new(0.103295e1) * t51871 - F::new(0.123954e2) * t51875 + F::cast_from(0.794188125e1_f64) * t51878 - F::cast_from(0.473371875e0_f64) * t51881 + F::new(0.94674375e0) * t51884 - F::new(0.52945875e1) * t51887;
    (t52522, t52536)
}
