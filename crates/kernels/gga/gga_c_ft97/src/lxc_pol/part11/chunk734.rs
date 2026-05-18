//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 734/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk734<F: Float>(t1775: F, t2489: F, t2508: F, t458: F, t192: F, t743: F, t9692: F, t462: F, t92: F, t9931: F, t9933: F, t9935: F, t9936: F, t9939: F, t9944: F, t9949: F, t9955: F, t9958: F) -> (F, F) {
    let t9960 = t1775 * t2489;
    let t9962 = t458 * t2508;
    let t9965 = t192 * t743 * t9692;
    let t9967 = t462 * t9931 + t9933 - t9935 - F::new(4.0) / F::new(3.0) * t9936 - t462 * t9939 / F::new(3.0) - F::new(6.0) * t92 * t9944 + F::new(6.0) * t462 * t9949 - F::new(10.0) / F::new(27.0) * t462 * t9955 + t9958 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t9960 - F::new(2.0) * t9962 - t92 * t9965;
    (t9965, t9967)
}
