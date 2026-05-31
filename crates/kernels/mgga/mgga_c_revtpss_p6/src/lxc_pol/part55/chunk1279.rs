//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1279/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1279<F: Float>(t119457: F, t122886: F, t122911: F, t122918: F, t124246: F, t124255: F, t124256: F, t129157: F, t129160: F, t129165: F, t129169: F, t129193: F, t129213: F, t129216: F, t130848: F, t130858: F, t130862: F, t130866: F, t130882: F, t130893: F, t32798: F, t33265: F, t33270: F, t33277: F, t34410: F, t34761: F, t4241: F, t640: F, t644: F, t8442: F, t8621: F, t8881: F, t8882: F) -> F {
    let t130895 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t124246 - t124255 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t124256 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t130848 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t129157 * t8882 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t129160 * t8882 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t129165 * t8882 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t129169 * t8882 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t130858 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t130862 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t130866 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t122911 * t34761 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t122918 * t34761 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t32798 * t8621 * t8881 * t4241 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t129193 * t33265 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t34410 * t33277 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t129213 * t33270 - F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t129216 * t8442 * t130882 * t644 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t122886 * t119457 * t130882 * t640 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t130893;
    t130895
}
