//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta543<F: Float>(t3453: F, t5146: F, t3479: F, t5142: F, t1168: F, t3471: F, t12472: F, t1744: F, t1757: F, t3497: F, t1745: F, t1187: F, t5181: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16955, t16958, t16959, t16962, t16965, t16966, t16971, t16974, t16979) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2206::<F>(t3453, t5146, t3479, t5142, t1168, t3471, t12472, t1744, t1757, t3497, t1745, t1187, t5181);
    (t16955, t16958, t16959, t16962, t16965, t16966, t16971, t16974, t16979)
}
