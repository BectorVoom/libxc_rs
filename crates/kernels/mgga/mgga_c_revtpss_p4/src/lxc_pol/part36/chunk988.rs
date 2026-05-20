//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 988/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk988<F: Float>(t4311: F, t5999: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t23189: F, t9514: F, t9517: F, t9521: F, t9524: F) -> (F, F) {
    let t23191 = F::new(12.0) * t4311 * t5999;
    let t23192 = -t10568 - t23189 + t9514 - t9517 - t9521 + t10577 + t10582 - t10584 - t10586 + t23191 - t9524;
    (t23191, t23192)
}
