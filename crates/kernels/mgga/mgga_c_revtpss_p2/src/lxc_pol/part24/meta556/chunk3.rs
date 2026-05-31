//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1664/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1664<F: Float>(t41307: F, t51978: F, t77736: F, t88118: F, t88126: F, t88134: F, t88168: F, t88171: F, t88203: F, t88206: F, t88209: F, t88211: F, t88214: F, t88216: F) -> F {
    let t88218 = F::cast_from(0.198684e1_f64) * t88168 + F::cast_from(0.49671e0_f64) * t88171 + t41307 + F::cast_from(0.132456e1_f64) * t77736 + F::cast_from(0.12524296296296296297e1_f64) * t51978 - F::cast_from(0.20128333333333333334e1_f64) * t88118 + F::cast_from(0.72462e1_f64) * t88126 - F::cast_from(0.80513333333333333332e0_f64) * t88134 + F::cast_from(0.258925e1_f64) * t88203 - F::cast_from(0.485484375e1_f64) * t88206 - F::cast_from(0.11038e0_f64) * t88209 + F::cast_from(0.6189328125e-1_f64) * t88211 + F::cast_from(0.247573125e0_f64) * t88214 - F::cast_from(0.3883875e1_f64) * t88216;
    t88218
}
