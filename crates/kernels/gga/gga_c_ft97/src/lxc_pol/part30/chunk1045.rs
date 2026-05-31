//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1045/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1045<F: Float>(t24437: F, t2574: F, t27850: F, t6119: F, t27819: F, t27820: F, t33319: F, t150288: F, t150291: F, t150295: F, t150298: F, t150302: F, t150304: F, t150308: F, t150915: F, t150918: F, t150922: F, t150927: F, t150931: F, t150935: F, t150939: F) -> (F, F, F) {
    let t150943 = t24437 * t2574 * t6119 * t27850;
    let t150946 = t27819 * t2574 * t33319 * t27820;
    let t150948 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t150288 + t150291 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) * t150295 + F::cast_from(2.0_f64) * t150298 + F::cast_from(4.0_f64) * t150302 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t150304 + F::cast_from(3.0_f64) * t150308 - t150915 - F::cast_from(6.0_f64) * t150918 + F::cast_from(2.0_f64) * t150922 + t150927 + t150931 + t150935 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t150939 - t150943 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t150946;
    (t150943, t150946, t150948)
}
