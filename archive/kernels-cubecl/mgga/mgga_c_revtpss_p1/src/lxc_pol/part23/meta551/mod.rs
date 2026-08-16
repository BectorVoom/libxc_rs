//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta551<F: Float>(t6862: F, t72: F, t686: F, t10023: F, t1385: F, t6888: F, t10070: F, t10074: F, t1399: F, t14191: F, t14193: F, t14203: F, t14209: F, t14255: F, t1883: F, t213: F, t21981: F, t22005: F, t22009: F, t22016: F, t22307: F, t4118: F, t546: F, t5659: F, t5675: F, t5745: F, t5755: F, t5767: F, t6874: F, t820: F) -> (F, F, F, F, F) {
        let (t22314, t22315, t22316, t22321, t22325) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2108::<F>(t6862, t72, t686, t10023, t1385, t6888, t10070, t10074, t1399, t14191, t14193, t14203, t14209, t14255, t1883, t213, t21981, t22005, t22009, t22016, t22307, t4118, t546, t5659, t5675, t5745, t5755, t5767, t6874, t820);
    (t22314, t22315, t22316, t22321, t22325)
}
