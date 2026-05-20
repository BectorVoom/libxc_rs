//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1788/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1788<F: Float>(t6587: F, t6622: F, t1042: F, t12787: F, t1715: F, t20795: F, t20809: F, t3711: F, t44190: F, t5340: F, t57471: F, t5819: F, t6429: F, t6640: F, t6690: F, t70758: F, t71275: F, t71513: F, t82816: F, t83504: F, t83539: F, t83558: F, t83580: F) -> (F, F) {
    let t91199 = t6587 * t6622;
    let t91228 = -F::cast_from(0.2540682555144873302e-3_f64) * t57471 - t83504 / F::new(36.0) - F::cast_from(0.86891343385954666928e-1_f64) * t71513 * t6690 + F::cast_from(0.57165357490759649296e-3_f64) * t3711 * t1042 * t82816 * t1715 + F::cast_from(0.11433071498151929859e-2_f64) * t83539 + F::cast_from(0.85748036236139473944e-3_f64) * t3711 * t1042 * t20809 * t6429 - F::cast_from(0.16937883700965822014e-2_f64) * t83558 + F::cast_from(0.57165357490759649296e-3_f64) * t70758 + F::cast_from(0.17149607247227894789e-2_f64) * t83580 + F::cast_from(0.18292914397043087775e-1_f64) * t71275 * t6640 + F::cast_from(0.28582678745379824648e-2_f64) * t5340 * t12787 * t20795 * t44190 * t5819;
    (t91199, t91228)
}
