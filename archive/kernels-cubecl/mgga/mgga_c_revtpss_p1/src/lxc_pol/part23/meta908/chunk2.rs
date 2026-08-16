//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2918/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2918<F: Float>(t6120: F, t918: F, t15107: F, t15110: F, t41246: F, t77499: F, t77503: F, t77505: F, t77683: F, t77686: F, t77688: F, t77690: F, t77692: F, t77695: F) -> (F, F, F) {
    let t77697 = t6120 * t918;
    let t77698 = t15107 * t77697;
    let t77700 = t15110 * t77697;
    let t77705 = F::cast_from(0.82524375e-1_f64) * t77683 - F::cast_from(0.485484375e1_f64) * t77686 + F::cast_from(0.58258125e1_f64) * t77688 - F::cast_from(0.3883875e1_f64) * t77690 - F::cast_from(0.3883875e1_f64) * t77692 + F::cast_from(0.6189328125e-1_f64) * t77695 + t41246 + F::cast_from(0.58258125e1_f64) * t77698 - F::cast_from(0.1237865625e0_f64) * t77700 + F::cast_from(0.11182407407407407407e0_f64) * t77499 - F::cast_from(0.301925e0_f64) * t77503 + F::cast_from(0.10064166666666666667e0_f64) * t77505;
    (t77698, t77700, t77705)
}
