//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1173/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1173(t10703: f64, t1248: f64, t143660: f64, t144197: f64, t144199: f64, t144212: f64, t144219: f64, t144227: f64, t144236: f64, t1476: f64, t15312: f64, t15369: f64, t154787: f64, t154794: f64, t154807: f64, t154813: f64, t154820: f64, t154827: f64, t154833: f64, t1901: f64, t2360: f64, t24898: f64, t28843: f64, t29245: f64, t296: f64, t33953: f64, t3886: f64, t4255: f64, t4260: f64, t446: f64, t6393: f64, t7021: f64, t7672: f64, t840: f64, t871: f64) -> f64 {
    let t154837 = t144197 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t144199 - 2.0_f64 / 9.0_f64 * t154787 - 2.0_f64 / 3.0_f64 * t446 * t840 * t6393 * t7021 - t446 * t296 * t154794 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t15312 * t143660 * t4260 - t144212 + t446 * t840 * t871 * t33953 * t1248 / 3.0_f64 - t144219 - 4.0_f64 / 9.0_f64 * t154807 - 2.0_f64 / 3.0_f64 * t446 * t840 * t28843 * t1476 - 4.0_f64 / 9.0_f64 * t154813 + 4.0_f64 / 9.0_f64 * t1901 * t15312 * t7672 * t2360 * t3886 + 2.0_f64 / 27.0_f64 * t154820 - 4.0_f64 / 3.0_f64 * t1901 * t15369 * t24898 * t29245 - 2.0_f64 / 9.0_f64 * t144227 + t154827 / 27.0_f64 + t144236 - t1901 * t10703 * t143660 * t4255 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t296 * t154833;
    t154837
}
