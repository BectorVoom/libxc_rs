//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1948;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1949;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta557<F: Float>(t114: F, t7898: F, t7937: F, t5542: F, t7934: F, t2014: F, t25826: F, t5891: F, t5915: F, t6998: F, t25822: F, t28679: F, t508: F, t651: F, t7935: F, t2022: F, t6895: F, t25924: F, t1903: F, t7910: F, t7296: F, t6918: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29993, t29996, t29998, t30004) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1948::<F>(t114, t7898, t7937, t5542, t7934, t2014, t25826, t5891, t5915, t6998, t25822, t28679);
        let (t30005, t30007, t30015, t30016, t30017, t30020, t30021, t30031) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1949::<F>(t30004, t508, t651, t7898, t7935, t2022, t6895, t25924, t1903, t7910, t7296, t6918);
    (t29993, t29996, t29998, t30004, t30005, t30007, t30015, t30016, t30017, t30020, t30021, t30031)
}
