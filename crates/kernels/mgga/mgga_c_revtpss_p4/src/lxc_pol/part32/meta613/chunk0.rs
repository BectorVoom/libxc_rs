//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1952/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1952<F: Float>(t18498: F, t27763: F, t106554: F, t27799: F, t18838: F, t33: F, t1353: F, t6922: F, t30105: F, t689: F, t1882: F, t543: F, t5774: F) -> (F, F, F, F, F, F) {
    let t108033 = t27763 * t18498;
    let t108036 = t27799 * t106554;
    let t108043 = t33 * t18838;
    let t108126 = t6922 * t1353;
    let t108138 = t30105 * t689;
    let t108178 = t5774 * t1882 * t543;
    (t108033, t108036, t108043, t108126, t108138, t108178)
}
