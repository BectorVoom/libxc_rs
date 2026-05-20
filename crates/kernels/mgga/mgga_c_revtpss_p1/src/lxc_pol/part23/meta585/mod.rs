//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta585<F: Float>(t23535: F, t916: F, t923: F, t1600: F, t6113: F, t11354: F, t11358: F, t11334: F, t11338: F, t18919: F, t18924: F, t18934: F, t19002: F, t19004: F, t19009: F, t23521: F, t23523: F) -> (F, F, F, F, F, F) {
        let (t23536, t23538, t23540, t23541, t23543, t23545) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2214::<F>(t23535, t916, t923, t1600, t6113, t11354, t11358, t11334, t11338, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523);
    (t23536, t23538, t23540, t23541, t23543, t23545)
}
