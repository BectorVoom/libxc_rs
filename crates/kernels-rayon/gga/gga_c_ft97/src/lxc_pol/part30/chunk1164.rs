//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1164/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1164(t1882: f64, t36236: f64, t25188: f64, t28847: f64, t36118: f64, t36215: f64, t36127: f64, t8392: f64, t36168: f64, t10703: f64, t1255: f64, t143858: f64, t1901: f64, t2347: f64, t25253: f64, t2862: f64, t28854: f64, t29128: f64, t29129: f64, t29193: f64, t296: f64, t33835: f64, t34012: f64, t34181: f64, t36121: f64, t36218: f64, t3886: f64, t4129: f64, t4167: f64, t4246: f64, t4311: f64, t446: f64, t56819: f64, t6278: f64, t684: f64, t7105: f64, t7131: f64, t7584: f64, t7611: f64, t7672: f64, t7679: f64, t840: f64, t871: f64, t99238: f64) -> (f64, f64) {
    let t154270 = t1882 * t36236;
    let t154285 = t25188 * t28847;
    let t154302 = t1882 * t36118;
    let t154304 = t1882 * t36215;
    let t154310 = t8392 * t36127;
    let t154327 = t1882 * t36168;
    let t154337 = 2.0_f64 / 3.0_f64 * t154270 + t446 * t840 * t871 * t7679 * t4129 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t2862 * t7131 * t6278 + t143858 + 2.0_f64 / 3.0_f64 * t446 * t840 * t25253 * t7105 + 4.0_f64 / 3.0_f64 * t446 * t296 * t154285 - t446 * t840 * t4311 * t7611 / 3.0_f64 - 4.0_f64 / 27.0_f64 * t1901 * t56819 * t7672 * t2347 * t3886 - t1901 * t10703 * t36121 * t684 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t154302 - 2.0_f64 / 9.0_f64 * t154304 - 4.0_f64 * t1901 * t29128 * t29129 * t28854 - t154310 / 27.0_f64 + t446 * t840 * t34012 * t4167 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t2862 * t4246 * t34181 + 4.0_f64 / 3.0_f64 * t446 * t2862 * t1255 * t33835 - 2.0_f64 / 9.0_f64 * t1901 * t99238 * t29193 + t154327 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t10703 * t36218 * t684 + 2.0_f64 / 3.0_f64 * t446 * t2862 * t4311 * t7584;
    (t154285, t154337)
}
