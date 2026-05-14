//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 958/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk958<F: Float>(t121093: F, t27888: F, t121019: F, t32284: F, t5700: F, t121018: F, t1399: F, t33962: F, t33955: F, t686: F, t72: F, t32705: F, t34230: F, t4075: F, t7063: F, t32240: F) -> (F, F, F, F, F, F, F) {
    let t125596 = t121093 * t27888;
    let t125599 = t32284 * t121019 * t5700;
    let t125603 = t121018 * t121019 * t33962 * t1399;
    let t125606 = t33955 * t72 * t686;
    let t125607 = t32705 * t125606;
    let t125609 = t34230 * t4075;
    let t125610 = t7063 * t125609;
    let t125611 = t125610 * t32240;
    (t125596, t125599, t125603, t125606, t125607, t125609, t125611)
}
