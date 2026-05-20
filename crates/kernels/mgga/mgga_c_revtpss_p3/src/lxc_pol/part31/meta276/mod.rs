//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1238;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1239;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta276<F: Float>(t7301: F, t7925: F, t545: F, t7910: F, t2028: F, t1904: F, t2027: F, t2030: F, t213: F, t561: F, t7245: F, t7248: F, t7279: F, t7288: F, t7291: F, t7295: F, t7911: F, t7917: F, t7921: F, t532: F, t1450: F, t2014: F, t2034: F, t5542: F, t118: F, t1502: F, t1519: F, t1843: F, t1911: F, t1932: F, t2007: F, t2011: F, t508: F, t569: F, t651: F, t6985: F, t7725: F, t7731: F, t7734: F, t7737: F, t7744: F, t7746: F, t7883: F, t7894: F, t7899: F, t7903: F, t3: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7926, t7929, t7930, t7933) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1238::<F>(t7301, t7925, t545, t7910, t2028, t1904, t2027, t2030, t213, t561, t7245, t7248, t7279, t7288, t7291, t7295, t7911, t7917, t7921);
        let (t7934, t7935, t7937, t7939) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1239::<F>(t532, t7933, t1450, t2014, t2034, t5542, t118, t1502, t1519, t1843, t1911, t1932, t2007, t2011, t508, t569, t651, t6985, t7725, t7731, t7734, t7737, t7744, t7746, t7883, t7894, t7899, t7903);
        let (t7940, t7944) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1240::<F>(t3, t7939, param_d);
    (t7926, t7929, t7930, t7933, t7934, t7935, t7937, t7939, t7940, t7944)
}
