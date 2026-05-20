//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1474/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1474<F: Float>(t18498: F, t2477: F, t828: F, t5984: F, t775: F, t800: F, t5988: F, t1548: F, t4343: F, t10811: F, t6037: F, t18444: F, t4364: F, t4366: F) -> (F, F, F, F, F, F) {
    let t18500 = t2477 * t828 * t18498;
    let t18507 = t800 * t5984 * t775;
    let t18511 = t800 * t5988 * t775;
    let t18515 = t800 * t1548 * t4343;
    let t18518 = t10811 * t6037;
    let t18521 = t4364 * t18444 * t4366;
    (t18500, t18507, t18511, t18515, t18518, t18521)
}
