//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1952;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta386<F: Float>(t13652: F, t1317: F, t5569: F, t3829: F, t566: F, t13640: F, t13641: F, t13643: F, t13644: F, t13645: F, t13646: F, t13647: F, t13648: F, t1448: F, t1868: F, t198: F, t4139: F, t4140: F, t5541: F, t5591: F, t9514: F, t9517: F, t9521: F, t9555: F, t9569: F, t9574: F, t9577: F, t9588: F, t9597: F, t123: F, t1856: F) -> (F, F, F, F, F, F) {
        let (t13653, t13655, t13656, t13663) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1952::<F>(t13652, t1317, t5569, t3829, t566, t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t1448, t1868, t198, t4139, t4140, t5541, t5591, t9514, t9517, t9521, t9555, t9569, t9574, t9577, t9588);
        let (t13664, t13665) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1953::<F>(t9597, t123, t1856);
    (t13653, t13655, t13656, t13663, t13664, t13665)
}
