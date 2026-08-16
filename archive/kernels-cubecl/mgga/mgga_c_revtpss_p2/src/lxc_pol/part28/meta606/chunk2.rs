//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2100/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2100<F: Float>(t25899: F, t97966: F, t25950: F, t27888: F, t25953: F, t27884: F, t13739: F, t13743: F, t25921: F, t27896: F, t28012: F, t7279: F, t7292: F, t7926: F, t94610: F, t94761: F, t94766: F, t94769: F, t94772: F, t94774: F, t94777: F) -> F {
    let t97974 = F::cast_from(0.25702851531048074406e-1_f64) * t25899 * t97966;
    let t97976 = F::cast_from(0.25702851531048074406e-1_f64) * t25950 * t27888;
    let t97985 = t27884 * t25953;
    let t97994 = t97974 - t97976 - t94761 + F::cast_from(0.4336814094102599731e0_f64) * t94610 * t7926 - F::cast_from(0.25702851531048074406e-1_f64) * t94766 + F::cast_from(0.14456046980341999104e-1_f64) * t94769 - F::cast_from(0.68540937416128198418e-2_f64) * t94772 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t27896 - F::cast_from(0.25702851531048074406e-1_f64) * t94774 + F::cast_from(0.17135234354032049604e-1_f64) * t97985 - F::cast_from(0.39512695097613069591e1_f64) * t7279 * t13743 - F::cast_from(0.45699670022203476294e-2_f64) * t94777 - F::cast_from(0.8673628188205199462e0_f64) * t7292 * t28012 + F::cast_from(0.13170898365871023197e1_f64) * t7279 * t13739;
    t97994
}
