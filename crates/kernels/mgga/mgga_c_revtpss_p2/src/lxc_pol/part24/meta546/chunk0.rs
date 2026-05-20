//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1618/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1618<F: Float>(t157: F, t190: F, t87145: F, t49926: F, t49940: F, t76972: F, t61165: F, t39756: F, t39760: F, t39764: F, t39770: F, t39773: F, t39783: F, t39786: F) -> (F, F, F, F, F, F) {
    let t87640 = F::new(24.0) * t87145 * t157 * t190;
    let t87641 = F::cast_from(0.86748650402413918736e-1_f64) * t49926;
    let t87642 = F::cast_from(0.14035736694323150897e2_f64) * t49940;
    let t87643 = F::cast_from(0.73245789224026180216e-3_f64) * t76972;
    let t87644 = F::new(72.0) * t61165;
    let t87645 = t39756 + t39760 - t39764 + t87640 + t39770 - t87641 + t87642 + t39773 - t87643 + t87644 - t39783 - t39786;
    (t87640, t87641, t87642, t87643, t87644, t87645)
}
