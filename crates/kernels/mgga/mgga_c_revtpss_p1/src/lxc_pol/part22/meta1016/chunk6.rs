//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3514/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3514<F: Float>(t11710: F, t16089: F, t19706: F, t16095: F, t20095: F, t1011: F, t11675: F, t11703: F, t15599: F, t16102: F, t20079: F, t3091: F, t3162: F, t42328: F, t42710: F, t43082: F, t43085: F, t4915: F, t4919: F, t54142: F, t54147: F, t6092: F, t63244: F, t63248: F, t63306: F, t63353: F, t66187: F) -> F {
    let t66467 = t16089 * t11710 * t19706;
    let t66470 = t16095 * t11710 * t20095;
    let t66500 = F::cast_from(0.76220476654346199061e-3_f64) * t66467 + F::cast_from(0.76220476654346199061e-3_f64) * t66470 + F::cast_from(0.28582678745379824648e-3_f64) * t11675 * t20079 + F::cast_from(0.23818898954483187207e-3_f64) * t3091 * t11703 * t6092 * t15599 + F::cast_from(0.57165357490759649296e-3_f64) * t54142 - F::cast_from(0.19055119163586549765e-3_f64) * t54147 + F::cast_from(0.28582678745379824648e-3_f64) * t42328 * t66187 * t3162 * t16102 + t1011 * t4915 * t63244 / F::new(48.0) + t1011 * t4919 * t63306 / F::new(6.0) - t1011 * t4915 * t63248 / F::new(72.0) - t1011 * t4919 * t63353 / F::new(36.0) - F::cast_from(0.31758531939310916275e-4_f64) * t42710 - F::cast_from(0.57165357490759649296e-3_f64) * t43082 * t66187 * t43085;
    t66500
}
