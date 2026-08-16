//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2789/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2789<F: Float>(t10982: F, t1568: F, t9646: F, t252: F, t2769: F, t2782: F, t4533: F, t886: F, t10513: F, t15011: F, t15030: F, t15038: F, t2765: F, t2772: F, t41060: F, t41063: F, t41067: F, t4487: F, t4534: F, t51227: F, t51231: F, t51234: F, t51237: F, t51240: F, t51241: F) -> F {
    let t51246 = t9646 * t1568 * t10982;
    let t51251 = t2782 * t252 * t2769 * t4533 * t886;
    let t51253 = F::cast_from(0.91069445034239308175e-1_f64) * t41060 + F::cast_from(0.32927245914677557992e-1_f64) * t41063 + F::cast_from(0.58544643236296698114e-1_f64) * t41067 + F::cast_from(0.79025390195226139182e1_f64) * t2765 * t15030 + F::cast_from(0.39512695097613069591e1_f64) * t15011 * t2772 + F::cast_from(0.16463622957338778996e-1_f64) * t51227 - F::cast_from(0.19756347548806534796e1_f64) * t10513 * t4534 - F::cast_from(0.58544643236296698113e-1_f64) * t51231 + t51234 + F::cast_from(0.39512695097613069591e1_f64) * t10513 * t4487 - F::cast_from(0.26019841438354088051e-2_f64) * t51237 + t51240 + F::cast_from(0.11708928647259339623e0_f64) * t51241 + F::cast_from(0.39512695097613069591e1_f64) * t2765 * t15038 + F::cast_from(0.19637199382202157274e-3_f64) * t51246 - F::cast_from(0.65854491829355115984e-1_f64) * t51251;
    t51253
}
