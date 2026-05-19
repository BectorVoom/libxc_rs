//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1300/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1300<F: Float>(t2110: F, t8249: F, t2172: F, t8113: F, t2167: F, t8130: F, t127453: F, t129018: F, t129026: F, t129029: F, t129032: F, t131119: F, t2170: F, t28987: F, t28990: F, t32377: F, t573: F, t5805: F, t7557: F, t7696: F, t8124: F, t8245: F, t8905: F, param_d: F) -> (F, F, F, F) {
    let t131133 = t2110 * t8249;
    let t131134 = t8113 * t2172;
    let t131135 = t2167 * t8130;
    let t131148 = t131119 * t573 * param_d + F::new(6.0) * t2170 * t28987 + F::new(3.0) * t2170 * t28990 + F::new(3.0) * t5805 * t8905 + F::new(3.0) * t7557 * t8245 + F::new(6.0) * t7696 * t8124 + t127453 + t129018 + t129026 + t129029 + t129032 + t32377;
    (t131133, t131134, t131135, t131148)
}
