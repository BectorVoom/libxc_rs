//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 969/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk969<F: Float>(t7235: F, t8232: F, t7271: F, t32490: F, t8392: F, t463: F, t7264: F, t1882: F, t32577: F, t487: F, t7165: F, t1637: F, t7266: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t138288 = F::new(4.0) / F::new(27.0) * t8232 * t7235;
    let t138290 = F::new(8.0) / F::new(27.0) * t8232 * t7271;
    let t138296 = t8392 * t32490;
    let t138298 = t463 * t7264;
    let t138302 = t1882 * t32577;
    let t138307 = t487 * t7165;
    let t138361 = F::new(4.0) / F::new(27.0) * t89 * t1637 * t7266;
    (t138288, t138290, t138296, t138298, t138302, t138307, t138361)
}
