//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1392/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1392<F: Float>(t1882: F, t31864: F, t31859: F, t112790: F, t114211: F, t114238: F, t114244: F, t114247: F, t1248: F, t1508: F, t15191: F, t18123: F, t1901: F, t2749: F, t2857: F, t2862: F, t28719: F, t29063: F, t31857: F, t31867: F, t4256: F, t446: F, t4965: F, t5299: F, t6386: F, t6393: F, t835: F, t840: F, t871: F, t99199: F) -> (F,) {
    let t127950 = t1882 * t31864;
    let t127956 = t1882 * t31859;
    let t127984 = -t114211 + 2.0 / 9.0 * t1901 * t112790 * t4256 - t127950 / 9.0 - 2.0 / 3.0 * t446 * t2862 * t2749 * t31857 + 2.0 / 9.0 * t127956 - t114238 - t446 * t835 * t1508 * t18123 / 9.0 - 2.0 / 27.0 * t446 * t2857 * t6393 * t4965 - t114244 - 4.0 / 81.0 * t99199 - t114247 + 2.0 / 3.0 * t446 * t840 * t2749 * t31867 + 2.0 / 3.0 * t446 * t840 * t871 * t28719 * t1248 + t446 * t840 * t871 * t6386 * t5299 / 3.0 + 2.0 / 9.0 * t1901 * t15191 * t29063;
    (t127984,)
}
