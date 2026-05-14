//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1066/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1066<F: Float>(t39743: F, t7932: F, t7942: F, t1907: F, t618: F, t615: F, t33527: F, t557: F, t2127: F, t2149: F, t2331: F, t31916: F, t31926: F, t31955: F, t33535: F, t33557: F, t33575: F, t33586: F, t36515: F, t38771: F, t40215: F, t6438: F, t7931: F, t8400: F, t8402: F, t8791: F, t9033: F, t939: F) -> (F, F, F) {
    let t40608 = t7942 * t7932 * t39743;
    let t40619 = t1907 * t618;
    let t40620 = t615 * t40619;
    let t40633 = t33527 * t557;
    let t40635 = -t33557 - 0.8673628188205199462e0 * t40608 - 0.17347256376410398924e1 * t8400 * t9033 * t38771 - 0.39512695097613069591e1 * t2127 * t6438 - 0.65854491829355115987e0 * t31916 - 0.17347256376410398924e1 * t7931 * t33535 * t8402 + 0.8673628188205199462e0 * t40620 * t2149 + 0.69389025505641595696e1 * t33575 + 0.8673628188205199462e0 * t31926 - t33586 - 0.17347256376410398924e1 * t8400 * t939 * t2331 * t8791 + 0.26020884564615598386e1 * t8400 * t36515 * t40215 - 0.26020884564615598386e1 * t31955 - 0.13170898365871023197e1 * t40633;
    (t40619, t40620, t40635)
}
