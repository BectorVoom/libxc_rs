//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3865/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3865<F: Float>(t22068: F, t9765: F, t22052: F, t3989: F, t1399: F, t1410: F, t22118: F, t22274: F, t3924: F, t3934: F, t4012: F, t48798: F, t73345: F, t74269: F, t74271: F, t74277: F, t74279: F, t74281: F, t74288: F, t828: F, t9955: F) -> F {
    let t74290 = t9765 * t22068;
    let t74292 = t3989 * t22052;
    let t74298 = F::new(7.0) / F::new(6.0) * t74269 - F::new(7.0) / F::new(12.0) * t74271 - F::cast_from(0.42874018118069736972e-2_f64) * t3934 * t9955 * t22118 * t3924 - F::cast_from(0.22675591804667994221e-1_f64) * t74277 + F::cast_from(0.11337795902333997111e0_f64) * t74279 - F::cast_from(0.76220476654346199061e-4_f64) * t74281 + F::cast_from(0.51448821741683684366e-1_f64) * t3934 * t48798 * t22274 * t1399 - F::cast_from(0.4065600224742826258e-3_f64) * t74288 - F::cast_from(0.27104001498285508387e-2_f64) * t74290 + F::cast_from(0.80031500487063509014e-2_f64) * t74292 + F::cast_from(0.85748036236139473944e-2_f64) * t1410 * t4012 * t828 * t73345;
    t74298
}
