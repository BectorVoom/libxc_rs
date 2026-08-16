//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 875/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk875(t1063: f64, t1671: f64, t3082: f64, t3086: f64, t3091: f64, t3169: f64, t375: f64, t4783: f64, t4788: f64, t4792: f64, t4794: f64, t4798: f64, t4803: f64, t4808: f64, t4848: f64, t4883: f64, t4928: f64) -> f64 {
    let t4930 = 0.14291339372689912324e-3_f64 * t3091 * t4783 + 0.14291339372689912324e-3_f64 * t3091 * t4788 - t3082 - t3086 / 108.0_f64 + 0.14291339372689912324e-3_f64 * t4792 - 0.11433071498151929859e-2_f64 * t4794 * t375 + 0.21437009059034868486e-3_f64 * t4798 * t375 - 0.28582678745379824648e-3_f64 * t1063 * t4803 + 0.23818898954483187207e-3_f64 * t1063 * t4808 - 0.11433071498151929859e-2_f64 * t3169 * t1671 + t4848 + t4883 + t4928;
    t4930
}
