//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1738/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1738(t26198: f64, t12020: f64, t2091: f64, t5325: f64, t26200: f64, t3887: f64, t5353: f64, t1375: f64, t26184: f64, t26187: f64, t26191: f64, t26195: f64, t26204: f64, t26207: f64, t26212: f64, t26224: f64, t3758: f64, t5326: f64, t7194: f64, t7925: f64) -> (f64, f64, f64, f64) {
    let t26988 = 0.16449340668482264365e-1_f64 * t26198;
    let t26989 = t12020 * t2091;
    let t26990 = t26989 * t5325;
    let t26993 = 0.38381794893125283518e-1_f64 * t26200;
    let t26996 = t3887 * t2091 * t5353;
    let t27005 = 0.76763589786250567037e-1_f64 * t26184 - 0.3289868133696452873e-1_f64 * t26187 - 0.3289868133696452873e-1_f64 * t26191 - 0.3289868133696452873e-1_f64 * t26195 + t26988 - 6.0_f64 * t26224 * t26990 + t26993 - 0.16449340668482264365e-1_f64 * t26204 + 2.0_f64 * t1375 * t26996 + 2.0_f64 * t7194 * t5326 + 2.0_f64 * t3758 * t7925 - 0.16449340668482264365e-1_f64 * t26207 + 0.16449340668482264365e-1_f64 * t26212;
    (t26989, t26990, t26996, t27005)
}
