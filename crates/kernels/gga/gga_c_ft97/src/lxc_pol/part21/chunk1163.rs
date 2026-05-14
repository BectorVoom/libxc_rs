//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1163/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1163<F: Float>(t25846: F, t942: F, t1317: F, t1800: F, t28: F, t116289: F, t1564: F, t446: F, t116446: F, t7793: F, t116292: F, t3281: F, t4495: F, t5617: F, t1307: F, t15885: F) -> (F, F, F, F, F, F, F, F) {
    let t116543 = t25846 * t942;
    let t116546 = t1317 * t28 * t1800 * t116543;
    let t116549 = t446 * t1564 * t116289;
    let t116552 = t446 * t7793 * t116446;
    let t116555 = t3281 * t1564 * t116292;
    let t116557 = t5617 * t4495;
    let t116560 = t1317 * t28 * t1800 * t116557;
    let t116561 = t1307 * t15885;
    (t116543, t116546, t116549, t116552, t116555, t116557, t116560, t116561)
}
