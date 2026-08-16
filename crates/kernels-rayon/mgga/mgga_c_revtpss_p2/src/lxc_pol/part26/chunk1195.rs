//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1195/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1195(t30: f64, t265: f64, t393: f64, t96072: f64, t10326: f64, t2078: f64, t2258: f64, t26626: f64, t45: f64, t606: f64, t7449: f64, t95972: f64, t96016: f64, t1940: f64, t2071: f64, t2082: f64, t2403: f64, t25767: f64, t25784: f64, t26425: f64, t26585: f64, t26590: f64, t28291: f64, t28472: f64, t33: f64, t7428: f64, t7432: f64, t92822: f64, t94228: f64, t94231: f64, t94234: f64, t94240: f64, t94246: f64, t94259: f64, t94276: f64, t94280: f64, t94293: f64, t94297: f64, t94316: f64, t95954: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t96073 = piecewise3(t394, 0.0_f64, t96072);
    let t96083 = piecewise3(t120, t95972 + t96016, t96073 * t45 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t26626 * t606 + 3.0_f64 / 2.0_f64 * t7449 * t2258 + t2078 * t10326 / 2.0_f64);
    let t96121 = 3.0_f64 * t1940 * t26590 * t94316 + 3.0_f64 * t92822 * t2082 - 3.0_f64 / 2.0_f64 * t1940 * t7432 * t94276 + 9.0_f64 / 2.0_f64 * t2403 * t7428 * t25767 + 9.0_f64 / 2.0_f64 * t2403 * t2071 * t94293 - 3.0_f64 / 2.0_f64 * t1940 * t26585 * t25784 - 9.0_f64 / 2.0_f64 * t26425 * t94228 + t1940 * t95954 * t33 / 2.0_f64 + 9.0_f64 / 2.0_f64 * t2403 * t2071 * t94297 + 3.0_f64 * t28472 * t94234 + 9.0_f64 * t26425 * t94231 - 9.0_f64 * t28291 * t94240 - 9.0_f64 * t26425 * t94246 + 9.0_f64 * t28291 * t94280 - 9.0_f64 / 2.0_f64 * t26425 * t94259;
    (t96083, t96121)
}
