//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1454/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1454(t103291: f64, t103293: f64, t103304: f64, t103315: f64, t109206: f64, t109244: f64, t109283: f64, t109324: f64, t109356: f64, t109393: f64, t109432: f64, t109732: f64, t1238: f64, t1241: f64, t1716: f64, t1760: f64, t19234: f64, t21510: f64, t22003: f64, t24589: f64, t24601: f64, t24602: f64, t24615: f64, t27382: f64, t27444: f64, t27784: f64, t27785: f64, t27820: f64, t29536: f64, t29812: f64, t4945: f64, t5398: f64, t6146: f64, t7283: f64, t7300: f64, t8061: f64, t8088: f64, t94525: f64) -> f64 {
    let t109743 = 0.49348022005446793095e-1_f64 * t7283 * t7300 * t24615 * t22003 - 0.82246703342411321826e-2_f64 * t103291 + 0.24674011002723396548e-1_f64 * t7283 * t1716 * t103315 + 12.0_f64 * t19234 * t8061 + 0.36554090374405031922e-2_f64 * t103293 + 0.82246703342411321826e-2_f64 * t24589 * t24601 * t24602 * t5398 * t1760 + 0.18277045187202515961e-2_f64 * t94525 + 0.24674011002723396548e-1_f64 * t7283 * t6146 * t27382 + 0.82246703342411321826e-2_f64 * t24589 * t27820 * t29812 - 0.16449340668482264365e-1_f64 * t24589 * t24601 * t27444 * t21510 - 0.16449340668482264365e-1_f64 * t103304 - 6.0_f64 * t19234 * t8088 - t1238 * t1241 * (t109206 + t109244 + t109283 + t109324 + t109356 + t109393 + t109432 + t109732) - 18.0_f64 * t27784 * t27785 * t22003 + 6.0_f64 * t4945 * t29536;
    t109743
}
