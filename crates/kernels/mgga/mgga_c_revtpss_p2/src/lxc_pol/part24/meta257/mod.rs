//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta257<F: Float>(t15669: F, t378: F, t379: F, t994: F, t1695: F, t3268: F, t3302: F, t5332: F, t1716: F, t2435: F) -> (F, F, F, F, F) {
        let (t16600, t16603, t16604, t16695, t16706) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1025::<F>(t15669, t378, t379, t994, t1695, t3268, t3302, t5332, t1716, t2435);
    (t16600, t16603, t16604, t16695, t16706)
}
