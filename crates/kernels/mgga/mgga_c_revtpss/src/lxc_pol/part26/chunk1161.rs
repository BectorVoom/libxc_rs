//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1161/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1161<F: Float>(t26205: F, t6963: F, t45972: F, t7342: F, t10309: F, t26178: F, t25159: F, t606: F, t68: F, t2047: F, t92569: F, t2048: F, t25114: F, t25120: F, t26175: F, t26187: F, t603: F, t7343: F, t7352: F, t92568: F, t92581: F, t92658: F, t92662: F, t92672: F, t92674: F, t92692: F, t92711: F) -> F {
    let t95314 = t6963 * t26205;
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    let t95320 = t95319 * t25159;
    let t95334 = t606 * t68;
    let t95340 = t2047 * t92569;
    let t95343 = -F::new(176.0) / F::new(9.0) * t95314 - F::new(70.0) * t95316 * t92692 - F::new(80.0) * t95320 - F::new(2.0) / F::new(3.0) * t92674 * t2048 - F::new(2.0) * t25120 * t7352 - F::new(5.0) * t26187 * t25114 - F::new(2.0) * t92711 * t2048 - F::new(5.0) * t7343 * t92658 - F::new(5.0) / F::new(3.0) * t7343 * t92662 - F::new(2.0) * t603 * t95334 * t92672 + F::new(30.0) * t26175 * t92581 - F::new(60.0) * t92568 * t95340;
    t95343
}
