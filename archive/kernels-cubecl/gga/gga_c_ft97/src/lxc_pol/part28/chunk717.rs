//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 717/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk717<F: Float>(t23890: F, t23914: F, t23920: F, t24034: F, t24041: F, t27116: F, t27121: F, t27126: F, t27130: F, t27133: F, t27135: F, t27139: F) -> F {
    let t27376 = -t27116 / F::cast_from(3.0_f64) + t23890 / F::cast_from(18.0_f64) - t24034 - t23914 / F::cast_from(27.0_f64) + t23920 / F::cast_from(9.0_f64) - t27121 / F::cast_from(9.0_f64) + t27126 / F::cast_from(12.0_f64) + t27130 / F::cast_from(3.0_f64) + t27133 / F::cast_from(3.0_f64) - t27135 / F::cast_from(36.0_f64) - t24041 + t27139 / F::cast_from(18.0_f64);
    t27376
}
