//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1119/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1119<F: Float>(t8397: F, t9432: F, t8998: F, t9436: F, t2226: F, t40697: F, t9154: F, t119: F, t150: F, t187: F, t2146: F, t2147: F, t2241: F, t33320: F, t33321: F, t38487: F, t38489: F, t38493: F, t40620: F, t42181: F, t463: F, t7912: F, t9003: F, t9386: F, t9971: F, t9986: F, t9991: F) -> (F,) {
    let t42261 = t8397 * t9432;
    let t42263 = t8998 * t9436;
    let t42269 = t40697 * t2226;
    let t42280 = t8998 * t9154;
    let t42284 = 0.17347256376410398924e1 * t7912 * t9986 - 0.17347256376410398924e1 * t42261 - t38487 + 0.17347256376410398924e1 * t42263 + t38489 + 0.8673628188205199462e0 * t2146 * t2147 * t9971 * t463 - 0.8673628188205199462e0 * t42269 + 0.8673628188205199462e0 * t7912 * t9991 - t38493 + 0.8673628188205199462e0 * t9003 * t9386 - t33320 + 0.65854491829355115987e0 * t119 * t42181 * t150 * t187 + 0.8673628188205199462e0 * t33321 - 0.34694512752820797848e1 * t42280 + 0.4336814094102599731e0 * t40620 * t2241;
    (t42284,)
}
