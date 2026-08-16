//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1554/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1554<F: Float>(t1280: F, t16750: F, t3153: F, t5284: F, t5465: F, t1287: F, t1811: F, t3588: F, t13133: F, t1774: F, t1214: F, t5245: F) -> (F, F, F, F, F, F) {
    let t16751 = t1280 * t16750;
    let t16756 = t5284 * t3153;
    let t16757 = t16756 * t5465;
    let t16763 = t1811 * t3588 * t1287;
    let t16768 = t13133 * t1774;
    let t16771 = t5245 * t1214;
    (t16751, t16756, t16757, t16763, t16768, t16771)
}
