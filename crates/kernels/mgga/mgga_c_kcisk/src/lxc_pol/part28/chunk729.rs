//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 729/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk729<F: Float>(t748: F, t9062: F, t5330: F, t8780: F, t746: F, t741: F, t2579: F, t2586: F, t1948: F, t5322: F, t8946: F, t5321: F, t2452: F, t651: F, t742: F, t79: F) -> (F, F, F, F, F, F, F, F) {
    let t9063 = t9062 * t748;
    let t9065 = t5330 * t8780;
    let t9066 = t746 * t9065;
    let t9067 = t741 * t9066;
    let t9069 = t2586 * t2579;
    let t9070 = t1948 * t9069;
    let t9072 = t5322 * t8946;
    let t9073 = t5321 * t9072;
    let t9077 = 1.0 / t651 / t742 / t2452;
    let t9078 = t9077 * t79;
    (t9063, t9066, t9067, t9069, t9070, t9072, t9073, t9078)
}
