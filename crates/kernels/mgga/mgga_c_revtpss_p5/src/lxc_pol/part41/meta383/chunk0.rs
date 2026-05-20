//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1266/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1266<F: Float>(t1043: F, t12050: F, t357: F, t19450: F, t6244: F, t999: F, t1082: F, t6234: F, t993: F, t225: F, t18902: F, t19025: F, t19027: F, t19029: F, t19031: F, t19048: F, t19051: F, t19053: F, t19055: F, t19058: F, t19060: F, t19062: F, t19079: F, t19081: F, t19084: F, t19130: F, t19132: F) -> (F, F, F, F, F, F) {
    let t19452 = t12050 * t1043 * t357;
    let t19453 = t19450 * t19452;
    let t19456 = t6244 * t999;
    let t19457 = t1082 * t19456;
    let t19462 = t6234 * t993;
    let t19463 = t19462 * t225;
    let t19466 = -t18902 - t19025 - t19027 - t19029 + t19031 + t19048 - t19051 - t19053 + t19055 + t19058 + t19060 + t19062 - t19079 - t19081 - t19084 + t19130 + t19132;
    (t19453, t19456, t19457, t19462, t19463, t19466)
}
