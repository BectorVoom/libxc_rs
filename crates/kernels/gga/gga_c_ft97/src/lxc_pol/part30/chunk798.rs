//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 798/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk798<F: Float>(t193: F, t34024: F, t33818: F, t33845: F, t33815: F, t33825: F, t33833: F, t33838: F, t33842: F, t33850: F, t33854: F, t33857: F, t33862: F) -> (F, F, F, F) {
    let t34025 = t193 * t34024;
    let t34031 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33818;
    let t34036 = t33845 / F::cast_from(3.0_f64);
    let t34041 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t33815 + t34031 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33825 + F::cast_from(4.0_f64) * t33833 - F::cast_from(2.0_f64) * t33838 - t33842 / F::cast_from(2.0_f64) - t34036 - t33850 / F::cast_from(3.0_f64) - F::cast_from(3.0_f64) * t33854 + F::cast_from(2.0_f64) * t33857 + t33862 / F::cast_from(4.0_f64);
    (t34025, t34031, t34036, t34041)
}
