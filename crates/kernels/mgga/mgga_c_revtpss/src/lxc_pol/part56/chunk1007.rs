//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1007/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1007<F: Float>(t1459: F, t34004: F, t105823: F, t572: F, t7002: F, t4292: F, t8453: F, t5795: F, t8614: F, t34007: F, t1916: F, t32366: F, t32855: F, t4248: F, t27123: F, t8749: F) -> (F, F, F, F, F, F, F, F) {
    let t127472 = 6.0 * t1459 * t34004;
    let t127480 = 12.0 * t572 * t105823 * t7002;
    let t127489 = 6.0 * t572 * t4292 * t8453;
    let t127495 = 3.0 * t5795 * t8614;
    let t127503 = 12.0 * t1459 * t34007;
    let t127507 = 6.0 * t1916 * t32366;
    let t129251 = t4248 * t32855;
    let t129253 = t27123 * t8749;
    (t127472, t127480, t127489, t127495, t127503, t127507, t129251, t129253)
}
