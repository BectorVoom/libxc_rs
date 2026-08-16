//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1239/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1239(t1113: f64, t2394: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25436: f64, t25440: f64, t25760: f64, t25763: f64, t25784: f64, t27158: f64, t27382: f64, t33: f64, t3351: f64, t4541: f64, t7087: f64, t7091: f64, t92819: f64, t93397: f64, t9357: f64, t94228: f64, t94231: f64, t94234: f64, t94240: f64, t94246: f64, t94255: f64, t94259: f64) -> f64 {
    let t94262 = t1113 * t2394;
    let t94272 = 3.0_f64 / 2.0_f64 * t1940 * t25436 * t1113 - 9.0_f64 / 2.0_f64 * t25206 * t94228 + 9.0_f64 * t25206 * t94231 + 3.0_f64 * t27382 * t94234 + t1940 * t1963 * t9357 / 2.0_f64 - 9.0_f64 * t27158 * t94240 - 9.0_f64 * t92819 * t25760 - 9.0_f64 * t25206 * t94246 + 3.0_f64 / 2.0_f64 * t1940 * t7087 * t3351 + t1940 * t93397 * t33 / 2.0_f64 - t1940 * t7091 * t94255 / 2.0_f64 - 9.0_f64 / 2.0_f64 * t25206 * t94259 + 9.0_f64 * t4541 * t1963 * t94262 + 9.0_f64 * t2403 * t7087 * t25763 - 3.0_f64 / 2.0_f64 * t1940 * t25440 * t25784;
    t94272
}
