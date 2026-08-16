//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1220/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1220<F: Float>(t39854: F, t37925: F, t37933: F, t39838: F, t39843: F, t39851: F, t39857: F, t39859: F, t39863: F, t39866: F, t39869: F, t41582: F) -> F {
    let t41584 = F::cast_from(0.13869154784086829701e1_f64) * t39854;
    let t41592 = F::cast_from(0.87327386630866483588e-2_f64) * t39838 - F::cast_from(0.26198215989259945076e-1_f64) * t39843 - t41582 - F::cast_from(0.13170898365871023197e1_f64) * t39851 - t41584 - F::cast_from(0.55476619136347318806e1_f64) * t39857 + F::cast_from(0.5200933044032561138e0_f64) * t39859 + F::cast_from(0.12805040077930161442e0_f64) * t37925 - F::cast_from(0.85366933852867742946e0_f64) * t37933 + F::cast_from(0.34672886960217074252e0_f64) * t39863 + F::cast_from(0.34672886960217074252e0_f64) * t39866 + F::cast_from(0.5200933044032561138e0_f64) * t39869;
    t41592
}
