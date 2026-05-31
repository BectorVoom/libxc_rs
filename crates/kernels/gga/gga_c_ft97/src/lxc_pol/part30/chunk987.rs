//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 987/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk987<F: Float>(t140535: F, t140556: F, t149743: F, t149748: F, t149750: F, t149753: F, t149760: F, t149764: F, t2354: F, t27971: F, t28010: F, t28015: F, t33269: F, t33502: F, t33504: F, t3746: F, t6005: F, t6745: F, t96834: F) -> F {
    let t149766 = -t140535 / F::cast_from(9.0_f64) - F::cast_from(24.0_f64) * t96834 * t27971 - t149743 * t6005 / F::cast_from(18.0_f64) + t6745 * t33269 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) * t149748 + t149750 / F::cast_from(27.0_f64) + F::cast_from(4.0_f64) * t149753 + t140556 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t28010 * t2354 * t33502 * t3746 + t149760 / F::cast_from(54.0_f64) - t28015 * t33504 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) * t149764;
    t149766
}
