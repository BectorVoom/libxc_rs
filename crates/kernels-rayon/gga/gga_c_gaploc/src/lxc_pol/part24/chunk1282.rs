//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1282/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1282(t28357: f64, t28361: f64, t11025: f64, t2087: f64, t4614: f64, t2610: f64, t7291: f64, t20019: f64, t8775: f64, t10978: f64, t5771: f64, t20671: f64, t24501: f64, t28309: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33080 = 0.63904876589867916128e-1_f64 * t28357;
    let t33081 = 0.15976219147466979032e0_f64 * t28361;
    let t33084 = 0.18404604457881959845e2_f64 * t2087 * t4614 * t11025;
    let t33087 = t2610 * t7291;
    let t33090 = 0.55611873258433997041e0_f64 * t8775 * t20019 * t33087;
    let t33092 = 0.14300195980740170668e1_f64 * t5771 * t10978;
    let t33094 = t28309 * t20671 * t24501;
    (t33080, t33081, t33084, t33090, t33092, t33094)
}
