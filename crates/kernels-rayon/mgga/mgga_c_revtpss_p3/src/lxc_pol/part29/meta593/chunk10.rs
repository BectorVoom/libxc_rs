//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1987/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1987(t102615: f64, t102617: f64, t102622: f64, t102629: f64, t102634: f64, t102636: f64, t14230: f64, t14269: f64, t25909: f64, t27868: f64, t28008: f64, t28899: f64, t28912: f64, t4078: f64, t7511: f64, t7532: f64, t8104: f64, t96516: f64, t96527: f64, t96542: f64, t96546: f64, t97855: f64) -> f64 {
    let t102642 = -0.23131639038696784278e-2_f64 * t96516 - t102615 + t102617 - 0.4336814094102599731e0_f64 * t25909 * t8104 + 0.13170898365871023197e1_f64 * t28899 * t4078 - 0.17347256376410398924e1_f64 * t27868 * t102622 * t14230 + 0.14456046980341999104e-1_f64 * t96527 - 0.65854491829355115987e0_f64 * t7511 * t14269 - 0.17135234354032049604e-2_f64 * t102629 - 0.8673628188205199462e0_f64 * t28008 * t7532 + t102634 - 0.24093411633903331839e-3_f64 * t102636 - 0.17347256376410398924e1_f64 * t97855 * t28912 - 0.14456046980341999104e-1_f64 * t96542 + 0.96373646535613327358e-3_f64 * t96546;
    t102642
}
