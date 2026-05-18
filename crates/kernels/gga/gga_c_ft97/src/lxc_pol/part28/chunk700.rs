//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 700/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk700<F: Float>(t23890: F, t23899: F, t23914: F, t23920: F, t23924: F, t27116: F, t27121: F, t27126: F, t27130: F, t27133: F, t27135: F, t27139: F) -> F {
    let t27141 = -t27116 + t23890 / F::new(6.0) - t23899 - t23914 / F::new(9.0) + t23920 / F::new(3.0) - t27121 / F::new(3.0) + t27126 / F::new(4.0) + t27130 + t27133 - t27135 / F::new(12.0) - t23924 + t27139 / F::new(6.0);
    t27141
}
