//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta493<F: Float>(t1168: F, t6487: F, t1745: F, t5142: F, t6506: F, t6503: F, t3479: F, t6502: F, t5146: F, t12472: F, t6486: F, t1130: F, t6433: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20606, t20609, t20612, t20615, t20618, t20619, t20622, t20625, t20626, t20629) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1973::<F>(t1168, t6487, t1745, t5142, t6506, t6503, t3479, t6502, t5146, t12472, t6486, t1130, t6433);
    (t20606, t20609, t20612, t20615, t20618, t20619, t20622, t20625, t20626, t20629)
}
