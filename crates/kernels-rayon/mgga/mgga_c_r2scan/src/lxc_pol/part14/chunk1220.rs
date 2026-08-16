//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1220/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1220(t39854: f64, t37925: f64, t37933: f64, t39838: f64, t39843: f64, t39851: f64, t39857: f64, t39859: f64, t39863: f64, t39866: f64, t39869: f64, t41582: f64) -> f64 {
    let t41584 = 0.13869154784086829701e1_f64 * t39854;
    let t41592 = 0.87327386630866483588e-2_f64 * t39838 - 0.26198215989259945076e-1_f64 * t39843 - t41582 - 0.13170898365871023197e1_f64 * t39851 - t41584 - 0.55476619136347318806e1_f64 * t39857 + 0.5200933044032561138e0_f64 * t39859 + 0.12805040077930161442e0_f64 * t37925 - 0.85366933852867742946e0_f64 * t37933 + 0.34672886960217074252e0_f64 * t39863 + 0.34672886960217074252e0_f64 * t39866 + 0.5200933044032561138e0_f64 * t39869;
    t41592
}
