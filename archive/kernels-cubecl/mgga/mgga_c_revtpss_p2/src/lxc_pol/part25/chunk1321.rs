//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1321/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1321<F: Float>(t94453: F, t94489: F, t94532: F, t94572: F, t1445: F, t2439: F, t25916: F, t1358: F, t212: F, t26034: F, t689: F, t10147: F, t2027: F, t2028: F, t26084: F, t4132: F, t543: F, t545: F, t7279: F, t7295: F, t7301: F, t94378: F, t94388: F, t94392: F, t94399: F, t94405: F, t94409: F, t94411: F, t94413: F, t9659: F) -> (F, F) {
    let t94574 = t94453 + t94489 + t94532 + t94572;
    let t94580 = t2439 * t25916 * t1445;
    let t94584 = t689 * t212 * t26034 * t1358;
    let t94588 = -F::cast_from(0.28912093960683998208e-1_f64) * t94378 - F::cast_from(0.39512695097613069591e1_f64) * t7279 * t9659 - F::cast_from(0.51405703062096148814e-2_f64) * t94388 + F::cast_from(0.68549505033305214441e-2_f64) * t94392 + F::cast_from(0.86736281882051994623e-1_f64) * t94399 - F::cast_from(0.65854491829355115987e0_f64) * t7279 * t10147 - F::cast_from(0.21684070470512998656e-1_f64) * t94405 - t94409 + F::cast_from(0.29272321618148349057e-1_f64) * t94411 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t94413 * t543 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t2028 * t545 * t94574 + F::cast_from(0.19514881078765566038e-2_f64) * t94580 - F::cast_from(0.16463622957338778996e-1_f64) * t94584 - F::cast_from(0.19756347548806534796e1_f64) * t26084 * t4132;
    (t94574, t94588)
}
