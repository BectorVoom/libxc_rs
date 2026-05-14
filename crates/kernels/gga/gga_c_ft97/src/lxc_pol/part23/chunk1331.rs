//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1331/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1331<F: Float>(t1882: F, t31881: F, t31877: F, t31873: F, t5225: F, t6260: F, t31758: F, t10683: F, t113656: F, t113914: F, t113915: F, t1255: F, t15254: F, t1901: F, t2360: F, t2766: F, t28719: F, t31572: F, t31640: F, t319: F, t3886: F, t4141: F, t4261: F, t4266: F, t446: F, t5299: F, t6393: F, t7091: F, t7124: F, t824: F, t840: F, t871: F, t882: F) -> (F, F) {
    let t126477 = t1882 * t31881;
    let t126479 = t1882 * t31877;
    let t126481 = t1882 * t31873;
    let t126487 = t6260 * t5225;
    let t126500 = t1882 * t31758;
    let t126518 = -4.0 / 9.0 * t1901 * t15254 * t7124 * t2360 * t3886 - 4.0 / 9.0 * t126477 - 2.0 / 9.0 * t126479 - 2.0 / 9.0 * t126481 - 2.0 * t446 * t10683 * t882 * t31572 - 2.0 * t446 * t10683 * t319 * t126487 - 2.0 / 3.0 * t446 * t840 * t1255 * t28719 - t446 * t840 * t6393 * t5299 / 3.0 + 2.0 / 9.0 * t126500 + 2.0 / 9.0 * t1901 * t113656 * t4261 + 4.0 / 9.0 * t1901 * t113656 * t4266 - 4.0 / 27.0 * t1901 * t2766 * t7091 * t4141 + t113914 - 8.0 / 27.0 * t113915 + t446 * t840 * t871 * t31640 * t824 / 3.0;
    (t126487, t126518)
}
