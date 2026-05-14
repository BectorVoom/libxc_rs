//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 399/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk399<F: Float>(t2097: F, t225: F, t561: F, t545: F, t2028: F, t2027: F, t213: F) -> (F, F, F, F) {
    let t2098 = t2097 * t225;
    let t2099 = t2098 * t561;
    let t2102 = t545 * t2097;
    let t2103 = t2028 * t2102;
    let t2106 = 0.65854491829355115987e0 * t213 * t2099 - 0.4336814094102599731e0 * t2027 * t2103;
    (t2098, t2102, t2103, t2106)
}
