//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 829/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk829<F: Float>(t18556: F, t10566: F, t23104: F, t23106: F, t23110: F, t23123: F, t23127: F, t23128: F, t23129: F, t23130: F, t9394: F, t18563: F, t4311: F, t5999: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t9514: F, t9517: F, t9521: F, t9524: F) -> (F, F, F, F, F) {
    let t23186 = 0.54934341918019635162e-3 * t18556;
    let t23187 = -t23104 - t23106 + t23110 + t23123 + t23127 + t23128 + t23129 + t9394 + t23130 + t10566 - t23186;
    let t23189 = 0.17544670867903938621e1 * t18563;
    let t23191 = 12.0 * t4311 * t5999;
    let t23192 = -t10568 - t23189 + t9514 - t9517 - t9521 + t10577 + t10582 - t10584 - t10586 + t23191 - t9524;
    (t23186, t23187, t23189, t23191, t23192)
}
