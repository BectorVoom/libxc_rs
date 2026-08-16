//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1223/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1223(t30592: f64, t32515: f64, t32517: f64, t34435: f64, t34449: f64, t34453: f64, t34468: f64, t34476: f64, t34478: f64, t34484: f64, t34485: f64, t34492: f64, t34499: f64, t37121: f64, t39330: f64, t39334: f64, t39337: f64, t39343: f64) -> f64 {
    let t41595 = 0.22921875e-1_f64 * t39330 + 0.22921875e-1_f64 * t39334 + 0.1528125e-1_f64 * t39337 + 0.18868855373762491241e-2_f64 * t34435 + t32515 + 0.9527559581793274883e-2_f64 * t30592 - 0.15724046144802076034e-2_f64 * t39343 + 0.25158473831683321656e-2_f64 * t34449 + 0.21437009059034868486e-2_f64 * t34453 - 0.18007087609589289529e-1_f64 * t34468 + 0.37737710747524982482e-2_f64 * t34476 + 0.2264262644851498949e-1_f64 * t34478 + t32517 - t34484 - t34485 + t37121 - 0.62896184579208304138e-3_f64 * t34492 - t34499;
    t41595
}
