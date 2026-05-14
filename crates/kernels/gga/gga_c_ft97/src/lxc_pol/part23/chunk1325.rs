//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1325/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1325<F: Float>(t2843: F, t5393: F, t6386: F, t10688: F, t31841: F, t112390: F, t4181: F, t15128: F, t28930: F, t31721: F, t8392: F, t112785: F, t113665: F, t125783: F, t15365: F, t15460: F, t1901: F, t19324: F, t19542: F, t29399: F, t296: F, t31890: F, t31894: F, t4167: F, t4246: F, t446: F, t56815: F, t6274: F, t6353: F, t71624: F, t840: F, t98809: F, t98850: F) -> (F, F, F, F, F) {
    let t126217 = t2843 * t6386 * t5393;
    let t126221 = t10688 * t31841;
    let t126225 = t112390 * t4181;
    let t126229 = t15128 * t28930;
    let t126236 = t8392 * t31721;
    let t126245 = -4.0 / 27.0 * t98850 - t446 * t296 * t125783 / 3.0 + 2.0 / 3.0 * t446 * t840 * t4246 * t29399 - 4.0 / 3.0 * t1901 * t15460 * t112785 * t4167 + t1901 * t71624 * t6274 / 9.0 - t113665 + 2.0 / 27.0 * t1901 * t98809 * t19542 + 2.0 / 3.0 * t446 * t296 * t126217 + 2.0 / 3.0 * t446 * t296 * t126221 + 4.0 / 3.0 * t446 * t296 * t126225 + 4.0 / 3.0 * t446 * t296 * t126229 - 4.0 / 3.0 * t1901 * t56815 * t31894 + 2.0 / 27.0 * t126236 + 2.0 / 27.0 * t1901 * t15365 * t31890 + 2.0 / 3.0 * t446 * t840 * t6353 * t19324;
    (t126217, t126221, t126225, t126229, t126245)
}
