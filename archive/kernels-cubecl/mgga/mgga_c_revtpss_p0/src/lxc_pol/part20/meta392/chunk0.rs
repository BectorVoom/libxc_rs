//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1445/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1445<F: Float>(t41372: F, t916: F, t270: F, t276: F, t39484: F, t41383: F, t2880: F, t41386: F, t11318: F, t698: F, t141: F, t41314: F, t930: F) -> (F, F, F, F, F) {
    let t41396 = t916 * t41372;
    let t41401 = F::cast_from(1.0_f64) / t276 / t39484 / t270 / F::cast_from(96.0_f64);
    let t41402 = t41401 * t41383;
    let t41404 = t2880 * t41386;
    let t41406 = t698 * t11318;
    let t41409 = t141 * t930 * t41314;
    (t41396, t41402, t41404, t41406, t41409)
}
