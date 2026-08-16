//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1247/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1247(t33071: f64, t10811: f64, t7751: f64, t326: f64, t32893: f64, t825: f64, t10906: f64, t2013: f64, t28357: f64, t28361: f64, t11025: f64, t2087: f64, t4614: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33072 = 0.59584149919750711116e-1_f64 * t33071;
    let t33074 = 0.42900587942220512003e1_f64 * t10811 * t7751;
    let t33077 = 0.92023022289409799224e1_f64 * t825 * t326 * t32893;
    let t33079 = 0.18404604457881959845e2_f64 * t2013 * t10906;
    let t33080 = 0.63904876589867916128e-1_f64 * t28357;
    let t33081 = 0.15976219147466979032e0_f64 * t28361;
    let t33084 = 0.18404604457881959845e2_f64 * t2087 * t4614 * t11025;
    (t33072, t33074, t33077, t33079, t33080, t33081, t33084)
}
