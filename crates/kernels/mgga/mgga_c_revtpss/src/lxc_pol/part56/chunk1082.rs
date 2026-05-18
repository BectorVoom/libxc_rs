//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1082/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1082<F: Float>(t33529: F, t3801: F, t12587: F, t8951: F, t44126: F, t8955: F, t2172: F, t7690: F, t2167: F, t7700: F, t1455: F, t8978: F) -> (F, F, F, F, F, F) {
    let t125070 = t33529 * t3801;
    let t125074 = t8951 * t12587;
    let t125092 = t8955 * t44126;
    let t125172 = t7690 * t2172;
    let t125174 = t2167 * t7700;
    let t125182 = t1455 * t8978;
    (t125070, t125074, t125092, t125172, t125174, t125182)
}
