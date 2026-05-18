//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 820/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk820<F: Float>(t2179: F, t33080: F, t574: F, t609: F, t7400: F, t9439: F, t144: F, t1384: F, t5968: F, t32895: F, t32922: F, t32892: F, t32902: F, t32910: F, t32915: F, t32919: F, t32927: F, t32931: F, t32935: F, t32940: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33082 = t574 * t2179 * t33080;
    let t33085 = t7400 * t609;
    let t33086 = t9439 * t33085;
    let t33087 = t144 * t33086;
    let t33090 = t1384 * t5968;
    let t33091 = t2179 * t33090;
    let t33092 = t144 * t33091;
    let t33096 = F::new(2.0) / F::new(9.0) * t32895;
    let t33101 = t32922 / F::new(9.0);
    let t33105 = t32892 / F::new(2.0) + t33096 + F::new(2.0) / F::new(9.0) * t32902 + F::new(4.0) / F::new(3.0) * t32910 - F::new(2.0) / F::new(3.0) * t32915 - t32919 / F::new(6.0) - t33101 - t32927 / F::new(9.0) - t32931 + F::new(2.0) / F::new(3.0) * t32935 + t32940 / F::new(12.0);
    (t33082, t33085, t33086, t33087, t33090, t33091, t33092, t33096, t33101, t33105)
}
