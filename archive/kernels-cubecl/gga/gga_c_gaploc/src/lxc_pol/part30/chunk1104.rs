//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1104/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1104<F: Float>(t7416: F, t9830: F, t10029: F, t2464: F, t2465: F, t2684: F, t7258: F, t22424: F, t3311: F, t161: F, t165: F, t7112: F) -> (F, F, F, F, F) {
    let t28820 = t7416 * t9830;
    let t28821 = F::cast_from(0.76685851907841499352e0_f64) * t28820;
    let t28822 = t7416 * t10029;
    let t28823 = F::cast_from(0.1022478025437886658e1_f64) * t28822;
    let t28827 = F::cast_from(0.17041300423964777634e0_f64) * t2684 * t2464 * t2465 * t7258;
    let t28828 = t22424 * t3311;
    let t28829 = F::cast_from(0.38342925953920749676e0_f64) * t28828;
    let t28831 = t161 * t165 * t7112;
    (t28821, t28823, t28827, t28829, t28831)
}
