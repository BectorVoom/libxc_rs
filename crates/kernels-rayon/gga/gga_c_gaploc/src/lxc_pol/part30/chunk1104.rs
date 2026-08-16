//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1104/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1104(t7416: f64, t9830: f64, t10029: f64, t2464: f64, t2465: f64, t2684: f64, t7258: f64, t22424: f64, t3311: f64, t161: f64, t165: f64, t7112: f64) -> (f64, f64, f64, f64, f64) {
    let t28820 = t7416 * t9830;
    let t28821 = 0.76685851907841499352e0_f64 * t28820;
    let t28822 = t7416 * t10029;
    let t28823 = 0.1022478025437886658e1_f64 * t28822;
    let t28827 = 0.17041300423964777634e0_f64 * t2684 * t2464 * t2465 * t7258;
    let t28828 = t22424 * t3311;
    let t28829 = 0.38342925953920749676e0_f64 * t28828;
    let t28831 = t161 * t165 * t7112;
    (t28821, t28823, t28827, t28829, t28831)
}
