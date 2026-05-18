//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 850/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk850<F: Float>(t2574: F, t265: F, t35323: F, t10157: F, t35318: F, t33291: F, t33318: F, t35312: F, t35316: F, t35321: F, t35326: F, t35330: F, t35334: F, t35338: F, t35341: F, t35346: F) -> (F, F, F) {
    let t35653 = t2574 * t265 * t35323;
    let t35657 = t10157 * t265 * t35318;
    let t35669 = F::new(3.0) / F::new(2.0) * t35312 + t33291 + F::new(2.0) / F::new(3.0) * t35316 + F::new(4.0) * t35321 - F::new(2.0) * t35326 - t35330 / F::new(2.0) - t33318 - t35334 / F::new(3.0) - F::new(3.0) * t35338 + F::new(2.0) * t35341 + t35346 / F::new(4.0);
    (t35653, t35657, t35669)
}
