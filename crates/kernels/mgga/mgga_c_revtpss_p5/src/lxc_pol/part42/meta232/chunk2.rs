//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 897/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk897<F: Float>(t6258: F, t996: F, t1592: F, t4823: F, t1042: F, t1469: F, t3094: F, t4781: F, t3092: F, t1651: F, t1668: F) -> (F, F, F, F, F, F, F) {
    let t6259 = t996 * t6258;
    let t6262 = t4823 * t1592;
    let t6263 = t1042 * t6262;
    let t6266 = t3094 * t1469;
    let t6267 = t4781 * t6266;
    let t6268 = t3092 * t6267;
    let t6271 = t1651 * t1668;
    (t6259, t6262, t6263, t6266, t6267, t6268, t6271)
}
