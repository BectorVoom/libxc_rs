//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 881/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk881<F: Float>(t37311: F, t7954: F, t92: F, t37357: F, t7763: F, t1642: F, t38042: F, t38044: F, t38046: F, t38048: F, t38050: F, t38055: F, t38059: F, t38063: F, t38066: F) -> (F, F, F, F) {
    let t38069 = t92 * t7954 * t37311;
    let t38071 = t7763 * t37357;
    let t38073 = t92 * t1642 * t38071;
    let t38075 = F::new(16.0) / F::new(9.0) * t38042 - F::new(16.0) / F::new(9.0) * t38044 + F::new(8.0) / F::new(9.0) * t38046 + F::new(8.0) / F::new(3.0) * t38048 - F::new(8.0) / F::new(3.0) * t38050 - F::new(80.0) / F::new(81.0) * t38055 - t38059 / F::new(3.0) + t38063 + F::new(40.0) / F::new(9.0) * t38066 - F::new(20.0) / F::new(9.0) * t38069 - F::new(8.0) * t38073;
    (t38069, t38071, t38073, t38075)
}
