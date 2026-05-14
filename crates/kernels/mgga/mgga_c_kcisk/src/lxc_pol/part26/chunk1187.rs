//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1187/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1187<F: Float>(t20634: F, t2152: F, t32045: F, t1411: F, t33604: F, t9814: F, t1339: F, t2722: F, t8162: F, t415: F, t8176: F, t9461: F, t2168: F, t2173: F, t32069: F, t6204: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t34723 = t20634 * t2152;
    let t34724 = t32045 * t34723;
    let t34725 = t1411 * t34724;
    let t34727 = t33604 * t9814;
    let t34728 = t1339 * t34727;
    let t34736 = t8162 * t2722;
    let t34737 = t415 * t34736;
    let t34739 = t9461 * t8176;
    let t34740 = t1339 * t34739;
    let t34742 = t2168 * t2173;
    let t34743 = t32069 * t34742;
    let t34744 = t6204 * t34743;
    (t34723, t34724, t34725, t34727, t34728, t34736, t34737, t34739, t34740, t34742, t34743, t34744)
}
