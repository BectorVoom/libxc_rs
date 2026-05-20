//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1514/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1514<F: Float>(t221: F, t23177: F, t2484: F, t2485: F, t1469: F, t4401: F, t61303: F, t14613: F, t18539: F, t18544: F, t4311: F, t23214: F, t750: F) -> (F, F, F, F, F) {
    let t76887 = t2484 * t2485 * t221 * t23177;
    let t76892 = t4401 * t61303 * t1469;
    let t76947 = t14613 * t18539;
    let t76949 = t4311 * t18544;
    let t76951 = t23214 * t750;
    (t76887, t76892, t76947, t76949, t76951)
}
