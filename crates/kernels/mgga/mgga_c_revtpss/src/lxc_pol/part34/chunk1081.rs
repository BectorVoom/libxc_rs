//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1081/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1081<F: Float>(t11940: F, t1972: F, t11735: F, t1968: F, t11772: F, t25515: F, t3114: F, t11240: F, t11244: F, t7120: F, t11627: F, t25503: F, t1976: F, t27639: F, t995: F, t25610: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93725 = t11940 * t1972;
    let t93750 = 5.0 / 1296.0 * t1968 * t11735;
    let t93751 = t25515 * t11772;
    let t93752 = t3114 * t93751;
    let t93758 = t11240 * t7120 * t11244;
    let t93789 = t11240 * t11627 * sigma0 * t11244;
    let t93793 = t11240 * t25503 * t11244;
    let t93870 = t11627 * t1976;
    let t93890 = t995 * t27639;
    let t93897 = t25610 * t27639;
    (t93725, t93750, t93752, t93758, t93789, t93793, t93870, t93890, t93897)
}
