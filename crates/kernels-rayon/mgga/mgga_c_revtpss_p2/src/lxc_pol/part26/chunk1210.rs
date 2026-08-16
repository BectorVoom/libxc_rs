//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1210/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1210(t96314: f64, t96329: f64, t96345: f64, t96360: f64, t26334: f64, t686: f64, t72: f64, t7289: f64, t7284: f64, t96282: f64, t2027: f64, t2028: f64, t2103: f64, t25909: f64, t26282: f64, t4078: f64, t545: f64, t7532: f64, t94643: f64, t96277: f64, t96280: f64, t96284: f64, t96287: f64, t96289: f64, t96292: f64, t96294: f64, t96296: f64, t96298: f64) -> (f64, f64, f64) {
    let t96362 = t96314 + t96329 + t96345 + t96360;
    let t96370 = t26334 * t72 * t686;
    let t96371 = t7289 * t96370;
    let t96374 = 0.22487184191643109717e-1_f64 * t7284 * t96282;
    let t96377 = -0.28912093960683998208e-1_f64 * t96277 - 0.10281140612419229763e-1_f64 * t96280 - t96284 - 0.13010442282307799193e1_f64 * t25909 * t7532 - 0.68549505033305214441e-2_f64 * t96287 + 0.51405703062096148812e-1_f64 * t96289 + 0.43368140941025997312e-1_f64 * t96292 - 0.77108554593144223218e-1_f64 * t96294 - 0.86736281882051994623e-1_f64 * t96296 + 0.28912093960683998208e-1_f64 * t96298 - 0.4336814094102599731e0_f64 * t2027 * t2028 * t545 * t96362 + 0.39512695097613069591e1_f64 * t26282 * t4078 - 0.38554277296572111609e-1_f64 * t96371 + t96374 - 0.4336814094102599731e0_f64 * t94643 * t2103;
    (t96362, t96370, t96377)
}
