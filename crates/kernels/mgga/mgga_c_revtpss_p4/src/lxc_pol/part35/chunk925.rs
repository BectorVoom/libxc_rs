//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 925/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk925<F: Float>(t1583: F, t6079: F, t10592: F, t10596: F, t10604: F, t10611: F, t11064: F, t198: F, t207: F, t23191: F, t23193: F, t23213: F, t23215: F, t23218: F, t23220: F, t23223: F, t9524: F, t9542: F) -> (F, F) {
    let t23429 = t6079 * t1583;
    let t23434 = F::new(2.0) * t11064 * t198 * t207 * t23429 + t10592 - t10596 - t10604 - t10611 + t23191 + t23193 + t23213 + t23215 + t23218 + t23220 + t23223 - t9524 + t9542;
    (t23429, t23434)
}
