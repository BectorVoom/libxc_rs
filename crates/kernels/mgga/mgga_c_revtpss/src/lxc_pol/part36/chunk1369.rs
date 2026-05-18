//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1369/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1369<F: Float>(t5883: F, t8151: F, t114434: F, t114436: F, t114438: F, t114440: F, t114442: F, t114445: F, t114451: F, t114455: F, t114746: F, t114753: F, t114755: F, t114757: F, t114759: F, t114765: F, t114768: F, t2163: F, t22578: F, t22634: F, t22639: F, t508: F, t5884: F, t7586: F, t8233: F) -> (F, F) {
    let t116732 = t8151 * t5883;
    let t116735 = -F::new(6.0) * t116732 * t508 - F::new(6.0) * t2163 * t22639 - F::new(6.0) * t22578 * t7586 - F::new(2.0) * t22634 * t7586 - F::new(6.0) * t5884 * t8233 - t114434 - t114436 - t114438 - t114440 - t114442 + t114445 + t114451 - t114455 + t114746 + t114753 + t114755 + t114757 - t114759 - t114765 + t114768;
    (t116732, t116735)
}
