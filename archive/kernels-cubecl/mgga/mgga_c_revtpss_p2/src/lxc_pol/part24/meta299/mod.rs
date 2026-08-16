//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta299<F: Float>(t1263: F, t6573: F, t1038: F, t6593: F, t1244: F, t1241: F, t5273: F, t5292: F, t1260: F, t6601: F, t140: F, t6652: F) -> (F, F, F, F, F, F) {
        let (t21093, t21101, t21102, t21107, t21143, t21169) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1084::<F>(t1263, t6573, t1038, t6593, t1244, t1241, t5273, t5292, t1260, t6601, t140, t6652);
    (t21093, t21101, t21102, t21107, t21143, t21169)
}
