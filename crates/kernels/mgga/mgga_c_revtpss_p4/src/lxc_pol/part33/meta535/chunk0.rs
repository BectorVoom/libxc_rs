//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1887/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1887<F: Float>(t30: F, t265: F, t393: F, t27254: F, t27256: F, t28034: F, t27924: F, t27926: F, t27929: F, t27937: F, t27955: F, t27754: F, t1469: F, t2129: F, t27408: F, t4186: F, t45: F, t606: F, t7594: F, t8161: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t28336 = F::cast_from(0.28582678745379824648e-4_f64) * t27254;
    let t28337 = F::cast_from(0.16006300097412701803e-1_f64) * t27256;
    let t28679 = F::new(2.0) / F::new(3.0) * t28034;
    let t28872 = F::cast_from(0.2032800112371413129e-3_f64) * t27924;
    let t28873 = F::cast_from(0.16006300097412701803e-1_f64) * t27926;
    let t28874 = F::cast_from(0.28582678745379824648e-4_f64) * t27929;
    let t28877 = F::cast_from(0.11433071498151929859e-3_f64) * t27937;
    let t28885 = F::new(7.0) / F::new(72.0) * t27955;
    let t28998 = piecewise3::<F>(t394, F::new(0.0), t27754);
    let t29005 = piecewise3::<F>(t120, t27408, t7594 * t1469 / F::new(2.0) + t2129 * t4186 / F::new(2.0) + t28998 * t45 / F::new(2.0) + t8161 * t606 / F::new(2.0));
    (t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t28998, t29005)
}
