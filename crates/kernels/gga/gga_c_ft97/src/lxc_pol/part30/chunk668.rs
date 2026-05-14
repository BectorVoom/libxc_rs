//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 668/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk668<F: Float>(t230: F, t709: F, t420: F, t1418: F, t6051: F, t7453: F, t33365: F, t3766: F) -> (F, F, F, F, F) {
    let t33373 = t230 * t709;
    let t33374 = t420 * t33373;
    let t33375 = t1418 * t33374;
    let t33379 = 0.25537443351851851852e-1 * t7453 * t6051;
    let t33380 = t3766 * t33365;
    (t33373, t33374, t33375, t33379, t33380)
}
