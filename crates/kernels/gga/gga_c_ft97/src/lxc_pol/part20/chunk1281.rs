//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1281/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1281<F: Float>(t29077: F, t8392: F, t1882: F, t29278: F, t29299: F, t29265: F, t681: F, t89: F, t2399: F, t7093: F, t28842: F, t870: F, t875: F, t7055: F, t8232: F, t10479: F, t1508: F, t15175: F, t15284: F, t15460: F, t1901: F, t2405: F, t25271: F, t2862: F, t29259: F, t296: F, t4261: F, t4311: F, t446: F, t6260: F, t840: F, t99034: F, t99703: F, t99706: F, t99712: F) -> (F, F) {
    let t114726 = 4.0 / 9.0 * t8392 * t29077;
    let t114728 = 2.0 / 9.0 * t1882 * t29278;
    let t114734 = 2.0 / 9.0 * t1882 * t29299;
    let t114747 = 2.0 / 9.0 * t89 * t681 * t29265;
    let t114749 = t89 * t2399 * t7093;
    let t114751 = t28842 * t870;
    let t114752 = t114751 * t875;
    let t114757 = t8232 * t7055;
    let t114762 = -4.0 / 3.0 * t1901 * t15460 * t25271 * t15284 + t114726 - t114728 + 2.0 / 27.0 * t1901 * t10479 * t29259 * t2405 - t114734 - 2.0 / 27.0 * t99703 - 2.0 / 27.0 * t99706 + 4.0 / 3.0 * t446 * t2862 * t1508 * t15175 - 2.0 / 3.0 * t446 * t840 * t4311 * t6260 - t114747 + 4.0 / 27.0 * t114749 - 2.0 / 3.0 * t446 * t296 * t114752 + t99712 / 9.0 - 4.0 / 81.0 * t114757 + 2.0 / 9.0 * t1901 * t99034 * t4261;
    (t114752, t114762)
}
