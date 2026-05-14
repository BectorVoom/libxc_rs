//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1031/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1031<F: Float>(t1882: F, t36175: F, t36002: F, t870: F, t875: F, t36246: F, t36211: F, t36157: F, t8392: F, t36257: F, t25253: F, t7124: F, t10703: F, t1248: F, t143660: F, t144197: F, t144199: F, t144212: F, t144219: F, t144227: F, t144236: F, t1476: F, t15312: F, t15369: F, t1901: F, t2360: F, t24898: F, t28843: F, t29245: F, t296: F, t33953: F, t3886: F, t4255: F, t4260: F, t446: F, t6393: F, t7021: F, t7672: F, t840: F, t871: F) -> (F, F, F) {
    let t154787 = t1882 * t36175;
    let t154793 = t36002 * t870;
    let t154794 = t154793 * t875;
    let t154807 = t1882 * t36246;
    let t154813 = t1882 * t36211;
    let t154820 = t8392 * t36157;
    let t154827 = t1882 * t36257;
    let t154833 = t25253 * t7124;
    let t154837 = t144197 / 9.0 + 2.0 / 9.0 * t144199 - 2.0 / 9.0 * t154787 - 2.0 / 3.0 * t446 * t840 * t6393 * t7021 - t446 * t296 * t154794 / 3.0 - 2.0 / 9.0 * t1901 * t15312 * t143660 * t4260 - t144212 + t446 * t840 * t871 * t33953 * t1248 / 3.0 - t144219 - 4.0 / 9.0 * t154807 - 2.0 / 3.0 * t446 * t840 * t28843 * t1476 - 4.0 / 9.0 * t154813 + 4.0 / 9.0 * t1901 * t15312 * t7672 * t2360 * t3886 + 2.0 / 27.0 * t154820 - 4.0 / 3.0 * t1901 * t15369 * t24898 * t29245 - 2.0 / 9.0 * t144227 + t154827 / 27.0 + t144236 - t1901 * t10703 * t143660 * t4255 / 9.0 - 2.0 / 3.0 * t446 * t296 * t154833;
    (t154794, t154833, t154837)
}
