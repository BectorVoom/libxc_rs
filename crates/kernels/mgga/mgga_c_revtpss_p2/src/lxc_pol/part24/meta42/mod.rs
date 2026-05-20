//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta42 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk294;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta42<F: Float>(t902: F, t307: F, t302: F, t928: F, t310: F, t320: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t939, t944, t945, t946, t948, t951, t954) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk294::<F>(t902, t307, t302, t928, t310);
        let (t958, t963, t964) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk295::<F>(t902, t320);
    (t939, t944, t945, t946, t948, t951, t954, t958, t963, t964)
}
