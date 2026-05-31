//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2929/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2929<F: Float>(t52546: F, t52547: F, t63240: F, t63242: F, t77663: F, t77667: F, t77670: F, t77672: F, t77674: F, t77676: F, t77679: F, t41672: F, t77499: F, t77503: F, t77505: F, t77683: F, t77686: F, t77688: F, t77690: F, t77692: F, t77695: F, t77698: F, t77700: F) -> (F, F) {
    let t77886 = t52546 - t52547 - F::cast_from(0.13892666666666666667e0_f64) * t77663 + F::cast_from(0.125034e1_f64) * t63240 - F::cast_from(0.83356000000000000002e0_f64) * t63242 + F::cast_from(0.30872592592592592593e-1_f64) * t77667 - F::cast_from(0.104195e0_f64) * t77670 - F::cast_from(0.473371875e0_f64) * t77672 + F::cast_from(0.94674375e0_f64) * t77674 + F::cast_from(0.94674375e0_f64) * t77676 - F::cast_from(0.17648625e1_f64) * t77679;
    let t77898 = F::cast_from(0.31558125e0_f64) * t77683 - F::cast_from(0.6618234375e1_f64) * t77686 + F::cast_from(0.794188125e1_f64) * t77688 - F::cast_from(0.52945875e1_f64) * t77690 - F::cast_from(0.52945875e1_f64) * t77692 + F::cast_from(0.2366859375e0_f64) * t77695 + t41672 + F::cast_from(0.794188125e1_f64) * t77698 - F::cast_from(0.473371875e0_f64) * t77700 + F::cast_from(0.19128703703703703704e0_f64) * t77499 - F::cast_from(0.516475e0_f64) * t77503 + F::cast_from(0.17215833333333333333e0_f64) * t77505;
    (t77886, t77898)
}
