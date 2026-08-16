//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1207/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1207(t39743: f64, t7932: f64, t7942: f64, t1907: f64, t618: f64, t615: f64, t33527: f64, t557: f64, t2127: f64, t2149: f64, t2331: f64, t31916: f64, t31926: f64, t31955: f64, t33535: f64, t33557: f64, t33575: f64, t33586: f64, t36515: f64, t38771: f64, t40215: f64, t6438: f64, t7931: f64, t8400: f64, t8402: f64, t8791: f64, t9033: f64, t939: f64) -> (f64, f64, f64) {
    let t40608 = t7942 * t7932 * t39743;
    let t40619 = t1907 * t618;
    let t40620 = t615 * t40619;
    let t40633 = t33527 * t557;
    let t40635 = -t33557 - 0.8673628188205199462e0_f64 * t40608 - 0.17347256376410398924e1_f64 * t8400 * t9033 * t38771 - 0.39512695097613069591e1_f64 * t2127 * t6438 - 0.65854491829355115987e0_f64 * t31916 - 0.17347256376410398924e1_f64 * t7931 * t33535 * t8402 + 0.8673628188205199462e0_f64 * t40620 * t2149 + 0.69389025505641595696e1_f64 * t33575 + 0.8673628188205199462e0_f64 * t31926 - t33586 - 0.17347256376410398924e1_f64 * t8400 * t939 * t2331 * t8791 + 0.26020884564615598386e1_f64 * t8400 * t36515 * t40215 - 0.26020884564615598386e1_f64 * t31955 - 0.13170898365871023197e1_f64 * t40633;
    (t40619, t40620, t40635)
}
