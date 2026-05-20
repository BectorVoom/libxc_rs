//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1217;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta353<F: Float>(t1668: F, t3154: F, t19572: F, t3117: F, t357: F, t15696: F, t6267: F, t23503: F, t4915: F, t11890: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t341: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23929, t23930, t23931, t23934, t23935, t23936, t23939, t23945, t23958) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1217::<F>(t1668, t3154, t19572, t3117, t357, t15696, t6267, t23503, t4915, t11890, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
        let t23959 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1218::<F>(t23958, t341);
    (t23929, t23930, t23931, t23934, t23935, t23936, t23939, t23945, t23958, t23959)
}
