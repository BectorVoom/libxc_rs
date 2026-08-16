//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1804/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1804<F: Float>(t1444: F, t7296: F, t8085: F, t8094: F, t25924: F, t2103: F, t25921: F, t26274: F, t26279: F, t26280: F, t26294: F, t26295: F, t26302: F, t26309: F, t27837: F, t28008: F, t5728: F, t7295: F, t7511: F, t7523: F, t7528: F, t8095: F) -> (F, F, F, F) {
    let t28806 = t7296 * t8085 * t1444;
    let t28814 = t8094 * t1444;
    let t28815 = t25924 * t28814;
    let t28822 = -F::cast_from(0.12851425765524037203e-1_f64) * t26274 + t26279 - F::cast_from(0.12851425765524037203e-1_f64) * t26280 - F::cast_from(0.4336814094102599731e0_f64) * t28008 * t2103 - t26294 + F::cast_from(0.25702851531048074406e-1_f64) * t26295 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t28806 + F::cast_from(0.54878743191129263322e-2_f64) * t26302 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t8095 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t7523 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t28815 + F::cast_from(0.4336814094102599731e0_f64) * t27837 * t7528 + t26309 + F::cast_from(0.13170898365871023197e1_f64) * t7511 * t5728;
    (t28806, t28814, t28815, t28822)
}
