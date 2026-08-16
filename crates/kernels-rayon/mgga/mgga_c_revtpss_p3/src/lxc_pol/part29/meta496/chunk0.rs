//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1804/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1804(t1444: f64, t7296: f64, t8085: f64, t8094: f64, t25924: f64, t2103: f64, t25921: f64, t26274: f64, t26279: f64, t26280: f64, t26294: f64, t26295: f64, t26302: f64, t26309: f64, t27837: f64, t28008: f64, t5728: f64, t7295: f64, t7511: f64, t7523: f64, t7528: f64, t8095: f64) -> (f64, f64, f64, f64) {
    let t28806 = t7296 * t8085 * t1444;
    let t28814 = t8094 * t1444;
    let t28815 = t25924 * t28814;
    let t28822 = -0.12851425765524037203e-1_f64 * t26274 + t26279 - 0.12851425765524037203e-1_f64 * t26280 - 0.4336814094102599731e0_f64 * t28008 * t2103 - t26294 + 0.25702851531048074406e-1_f64 * t26295 + 0.8673628188205199462e0_f64 * t7295 * t28806 + 0.54878743191129263322e-2_f64 * t26302 + 0.8673628188205199462e0_f64 * t25921 * t8095 + 0.8673628188205199462e0_f64 * t27837 * t7523 - 0.26020884564615598386e1_f64 * t7295 * t28815 + 0.4336814094102599731e0_f64 * t27837 * t7528 + t26309 + 0.13170898365871023197e1_f64 * t7511 * t5728;
    (t28806, t28814, t28815, t28822)
}
