//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1157/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1157(t37925: f64, t37933: f64, t39838: f64, t39843: f64, t39846: f64, t39851: f64, t39855: f64, t39858: f64, t39859: f64, t39863: f64, t39866: f64, t39869: f64) -> f64 {
    let t39871 = 0.43663693315433241792e-2_f64 * t39838 - 0.13099107994629972538e-1_f64 * t39843 - 0.42377972951376424087e0_f64 * t39846 - 0.65854491829355115988e0_f64 * t39851 - t39855 - t39858 + 0.2600466522016280569e0_f64 * t39859 + 0.64025200389650807209e-1_f64 * t37925 - 0.42683466926433871472e0_f64 * t37933 + 0.17336443480108537126e0_f64 * t39863 + 0.17336443480108537126e0_f64 * t39866 + 0.2600466522016280569e0_f64 * t39869;
    t39871
}
