//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1324;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta369<F: Float>(t9552: F, t9559: F, t1317: F, t5567: F, t9564: F, t9566: F, t9578: F, t9580: F, t4147: F, t5778: F, t2496: F, t5571: F, t5569: F, t3829: F, t566: F, t1448: F, t1868: F, t198: F, t4139: F, t4140: F, t5541: F, t5591: F, t9514: F, t9517: F, t9521: F, t9555: F, t9569: F, t9574: F, t9577: F, t9588: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t13652) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1324::<F>(t9552, t9559, t1317, t5567, t9564, t9566, t9578, t9580, t4147, t5778, t2496, t5571);
        let (t13653, t13655, t13663) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1325::<F>(t13652, t1317, t5569, t3829, t566, t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t1448, t1868, t198, t4139, t4140, t5541, t5591, t9514, t9517, t9521, t9555, t9569, t9574, t9577, t9588);
    (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t13653, t13655, t13663)
}
