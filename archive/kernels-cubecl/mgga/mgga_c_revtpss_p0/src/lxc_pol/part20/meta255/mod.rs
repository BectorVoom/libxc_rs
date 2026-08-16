//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1088;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta255<F: Float>(t276: F, t285: F, t2881: F, t918: F, t273: F, t2439: F, t931: F, t2915: F, t698: F, t11315: F, t916: F, t2880: F, t2889: F, t2897: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11354, t11356, t11358, t11359, t11366, t11368, t11370, t11372) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1088::<F>(t276, t285, t2881, t918, t273, t2439, t931, t2915, t698, t11315, t916, t2880);
        let (t11373, t11375, t11376, t11378) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1089::<F>(t11372, t2889, t2897, t918, t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368, t11370);
    (t11354, t11356, t11358, t11359, t11366, t11368, t11370, t11372, t11373, t11375, t11376, t11378)
}
