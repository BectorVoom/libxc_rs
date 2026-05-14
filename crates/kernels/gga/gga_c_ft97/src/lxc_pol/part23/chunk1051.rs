//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1051/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1051<F: Float>(t4246: F, t7105: F, t840: F, t1508: F, t4969: F, t835: F, t1476: F, t5424: F, t15299: F, t31564: F, t5413: F, t6334: F, t10703: F, t1901: F, t24955: F, t25194: F, t29232: F, t29238: F, t29285: F, t31752: F, t31758: F, t31762: F, t31766: F, t31770: F, t31774: F, t31779: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t31783 = t840 * t4246 * t7105;
    let t31787 = t835 * t1508 * t4969;
    let t31791 = t840 * t5424 * t1476;
    let t31795 = t15299 * t31564;
    let t31798 = t6334 * t5413;
    let t31799 = t10703 * t31798;
    let t31802 = 2.0 / 3.0 * t446 * t31752 + 2.0 / 9.0 * t29232 - t24955 - 2.0 / 27.0 * t29238 - 2.0 / 3.0 * t446 * t31758 - t446 * t31762 / 3.0 - t446 * t31766 / 3.0 - t446 * t31770 / 9.0 - 2.0 / 27.0 * t446 * t31774 - t25194 + 2.0 / 3.0 * t446 * t31779 + 2.0 / 3.0 * t446 * t31783 + 2.0 / 9.0 * t446 * t31787 - t446 * t31791 / 3.0 + 2.0 / 9.0 * t29285 - 4.0 / 9.0 * t1901 * t31795 - 2.0 / 9.0 * t1901 * t31799;
    (t31783, t31787, t31791, t31795, t31798, t31799, t31802)
}
