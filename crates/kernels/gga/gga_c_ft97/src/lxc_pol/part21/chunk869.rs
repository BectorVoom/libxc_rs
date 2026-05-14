//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 869/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk869<F: Float>(t86: F, t25568: F, t25852: F, t26054: F, t26496: F, t1342: F, t18: F, t113: F, t1577: F, t5: F, t505: F, t5756: F, t6570: F, t992: F, t1058: F, t5843: F, t28: F) -> (F, F, F, F) {
    let t87 = 10000000.0 <= t86;
    let t26498 = t25568 + t25852 + t26054 + t26496;
    let t26508 = t1342 * t18;
    let t26513 = piecewise3(t87, 0.0, t5 * t26498 * t113 / 4.0 + t5 * t6570 * t505 / 4.0 + t5 * t5756 * t992 / 4.0 - t5 * t26508 * t1577 / 2.0);
    let t26514 = t5843 * t1058;
    let t26515 = t28 * t26514;
    (t26498, t26513, t26514, t26515)
}
