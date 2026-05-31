//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1223/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1223<F: Float>(t30592: F, t32515: F, t32517: F, t34435: F, t34449: F, t34453: F, t34468: F, t34476: F, t34478: F, t34484: F, t34485: F, t34492: F, t34499: F, t37121: F, t39330: F, t39334: F, t39337: F, t39343: F) -> F {
    let t41595 = F::cast_from(0.22921875e-1_f64) * t39330 + F::cast_from(0.22921875e-1_f64) * t39334 + F::cast_from(0.1528125e-1_f64) * t39337 + F::cast_from(0.18868855373762491241e-2_f64) * t34435 + t32515 + F::cast_from(0.9527559581793274883e-2_f64) * t30592 - F::cast_from(0.15724046144802076034e-2_f64) * t39343 + F::cast_from(0.25158473831683321656e-2_f64) * t34449 + F::cast_from(0.21437009059034868486e-2_f64) * t34453 - F::cast_from(0.18007087609589289529e-1_f64) * t34468 + F::cast_from(0.37737710747524982482e-2_f64) * t34476 + F::cast_from(0.2264262644851498949e-1_f64) * t34478 + t32517 - t34484 - t34485 + t37121 - F::cast_from(0.62896184579208304138e-3_f64) * t34492 - t34499;
    t41595
}
