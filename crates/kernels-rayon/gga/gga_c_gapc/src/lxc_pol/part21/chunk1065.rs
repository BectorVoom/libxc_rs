//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1065/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1065(t190: f64, t2660: f64, t286: f64, t33235: f64, t442: f64, t8139: f64, t28924: f64, t3784: f64, t33209: f64, t33212: f64, t33214: f64, t33217: f64, t33221: f64, t33226: f64, t33228: f64, t33230: f64, t33232: f64) -> f64 {
    let t33240 = t2660 * t33235 * t8139 * t190 * t286 * t442;
    let t33242 = t3784 * t28924;
    let t33244 = 0.67528199161846004232e-6_f64 * t33209 + 0.18115908419564701086e-6_f64 * t33212 - 0.10129229874276900635e-5_f64 * t33214 + 0.90579542097823505428e-7_f64 * t33217 + 0.82779637083844259127e-6_f64 * t33221 + 0.59920486569434427612e-7_f64 * t33226 - 0.12650553385416666667e-5_f64 * t33228 + 0.9275345110817126956e-4_f64 * t33230 + 0.77294542590142724635e-6_f64 * t33232 - 0.12187980608940473897e-4_f64 * t33240 - 0.33147827249531850014e-7_f64 * t33242;
    t33244
}
