//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1429;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta378(t9552: f64, t9559: f64, t1317: f64, t5567: f64, t9564: f64, t9566: f64, t9578: f64, t9580: f64, t4147: f64, t5778: f64, t2496: f64, t5571: f64, t5569: f64, t3829: f64, t566: f64, t1448: f64, t1868: f64, t198: f64, t4139: f64, t4140: f64, t5541: f64, t5591: f64, t9514: f64, t9517: f64, t9521: f64, t9555: f64, t9569: f64, t9574: f64, t9577: f64, t9588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t13652) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1429(t9552, t9559, t1317, t5567, t9564, t9566, t9578, t9580, t4147, t5778, t2496, t5571);
        let (t13653, t13655, t13663) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1430(t13652, t1317, t5569, t3829, t566, t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t1448, t1868, t198, t4139, t4140, t5541, t5591, t9514, t9517, t9521, t9555, t9569, t9574, t9577, t9588);
    (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t13653, t13655, t13663)
}
