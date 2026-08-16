//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1446/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1446(t1188: f64, t14849: f64, t15083: f64, t17516: f64, t18191: f64, t277: f64, t3245: f64, t4281: f64, t4290: f64, t5229: f64, t52329: f64, t53399: f64, t53851: f64, t58322: f64, t58917: f64, t59086: f64, t59088: f64, t59152: f64, t59154: f64, t60243: f64, t95: f64) -> f64 {
    let t60249 = 800.0_f64 / 81.0_f64 * t14849 * t18191 + 136400.0_f64 / 729.0_f64 * t53399 * t5229 + 200.0_f64 / 3.0_f64 * t15083 * t17516 + 6.0_f64 * t4281 * t3245 * t4290 * t58322 + 16000000.0_f64 / 729.0_f64 * t52329 * t58917 + 0.25844881434903430496e-2_f64 * t95 * t277 * t60243 * t1188 + 200.0_f64 / 27.0_f64 * t53851 - t59086 + t59088 + t59152 + t59154;
    t60249
}
