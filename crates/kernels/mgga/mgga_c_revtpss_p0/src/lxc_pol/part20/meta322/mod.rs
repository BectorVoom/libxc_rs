//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta322<F: Float>(t1214: F, t2258: F, t5296: F, t1042: F, t3617: F, t3363: F, t3172: F, t3590: F, t1247: F, t11231: F, t5302: F, t3612: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12931, t12932, t12933, t12936, t12937, t12938, t12941, t12942, t12944, t12945, t12948) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1233::<F>(t1214, t2258, t5296, t1042, t3617, t3363, t3172, t3590, t1247, t11231, t5302, t3612);
    (t12931, t12932, t12933, t12936, t12937, t12938, t12941, t12942, t12944, t12945, t12948)
}
