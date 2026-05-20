//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3756/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3756<F: Float>(t1261: F, t20867: F, t3172: F, t12956: F, t20783: F, t3617: F, t6587: F, t17609: F, t5265: F, t17544: F, t5274: F, t1042: F, t1247: F, t1250: F, t13069: F, t17261: F, t17536: F, t17569: F, t17700: F, t20864: F, t20903: F, t21095: F, t3363: F, t3647: F, t3708: F, t3711: F, t482: F, t5381: F, t59149: F, t6625: F, t69609: F) -> F {
    let t71539 = t1261 * t3172 * t20867;
    let t71541 = t12956 * t20783;
    let t71543 = t3617 * t6587;
    let t71550 = t17609 * t5265;
    let t71552 = t5274 * t17544;
    let t71560 = F::cast_from(0.57165357490759649296e-3_f64) * t59149 + F::cast_from(0.21437009059034868486e-3_f64) * t13069 * t6625 + F::cast_from(0.42874018118069736972e-3_f64) * t3708 * t20903 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t1042 * t482 * t69609 * t1250 + F::cast_from(0.19055119163586549765e-2_f64) * t71539 + F::cast_from(0.3811023832717309953e-3_f64) * t71541 - F::cast_from(0.23818898954483187207e-3_f64) * t3711 * t1042 * t71543 * t3363 + F::cast_from(0.57165357490759649296e-3_f64) * t17569 * t17536 + F::cast_from(0.57165357490759649296e-3_f64) * t71550 + F::cast_from(0.57165357490759649296e-3_f64) * t71552 - F::cast_from(0.57165357490759649296e-3_f64) * t17261 * t21095 + F::cast_from(0.95275595817932748828e-3_f64) * t3647 * t20864 + F::cast_from(0.95275595817932748826e-3_f64) * t5381 * t17700;
    t71560
}
