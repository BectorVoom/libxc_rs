//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 986/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk986<F: Float>(t1042: F, t12952: F, t1260: F, t3666: F, t3172: F, t3713: F, t3711: F, t127: F, t3661: F, t371: F, t1235: F, t12640: F, t225: F, t480: F, t12621: F, t482: F) -> (F, F, F, F, F, F, F) {
    let t12953 = t1042 * t12952;
    let t12956 = t3666 * t1260;
    let t12959 = t3172 * t3713;
    let t12960 = t3711 * t12959;
    let t12963 = t371 * t127 * t3661;
    let t12964 = t1235 * t12963;
    let t12966 = t12640 * t225;
    let t12967 = t12966 * t480;
    let t12970 = t482 * t12621;
    (t12953, t12956, t12960, t12964, t12966, t12967, t12970)
}
