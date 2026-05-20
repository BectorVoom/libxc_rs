//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta329<F: Float>(t13044: F, t13063: F, t1042: F, t1032: F, t3552: F, t1246: F, t1250: F, t12732: F, t482: F, t1263: F, t3568: F, t1122: F) -> (F, F, F, F, F, F, F) {
        let (t13064, t13065, t13069, t13075, t13076, t13079, t13080) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1245::<F>(t13044, t13063, t1042, t1032, t3552, t1246, t1250, t12732, t482, t1263, t3568, t1122);
    (t13064, t13065, t13069, t13075, t13076, t13079, t13080)
}
