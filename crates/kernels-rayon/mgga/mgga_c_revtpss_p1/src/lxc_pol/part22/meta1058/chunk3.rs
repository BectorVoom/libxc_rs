//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3756/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3756(t1261: f64, t20867: f64, t3172: f64, t12956: f64, t20783: f64, t3617: f64, t6587: f64, t17609: f64, t5265: f64, t17544: f64, t5274: f64, t1042: f64, t1247: f64, t1250: f64, t13069: f64, t17261: f64, t17536: f64, t17569: f64, t17700: f64, t20864: f64, t20903: f64, t21095: f64, t3363: f64, t3647: f64, t3708: f64, t3711: f64, t482: f64, t5381: f64, t59149: f64, t6625: f64, t69609: f64) -> f64 {
    let t71539 = t1261 * t3172 * t20867;
    let t71541 = t12956 * t20783;
    let t71543 = t3617 * t6587;
    let t71550 = t17609 * t5265;
    let t71552 = t5274 * t17544;
    let t71560 = 0.57165357490759649296e-3_f64 * t59149 + 0.21437009059034868486e-3_f64 * t13069 * t6625 + 0.42874018118069736972e-3_f64 * t3708 * t20903 + 0.21437009059034868486e-3_f64 * t1247 * t1042 * t482 * t69609 * t1250 + 0.19055119163586549765e-2_f64 * t71539 + 0.3811023832717309953e-3_f64 * t71541 - 0.23818898954483187207e-3_f64 * t3711 * t1042 * t71543 * t3363 + 0.57165357490759649296e-3_f64 * t17569 * t17536 + 0.57165357490759649296e-3_f64 * t71550 + 0.57165357490759649296e-3_f64 * t71552 - 0.57165357490759649296e-3_f64 * t17261 * t21095 + 0.95275595817932748828e-3_f64 * t3647 * t20864 + 0.95275595817932748826e-3_f64 * t5381 * t17700;
    t71560
}
