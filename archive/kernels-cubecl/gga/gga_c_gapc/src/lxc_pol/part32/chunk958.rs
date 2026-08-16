//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 958/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk958<F: Float>(t3729: F, t828: F, t11614: F, t11617: F, t11621: F, t11623: F, t11627: F, t11630: F, t11634: F, t11638: F, t11641: F, t11644: F, t11649: F, t11651: F) -> F {
    let t11653 = t828 * t3729;
    let t11655 = -F::cast_from(0.82073827867876094584e-5_f64) * t11614 - F::cast_from(0.82073827867876094584e-5_f64) * t11617 + F::cast_from(0.11742981196020707897e-5_f64) * t11621 - F::cast_from(0.80732995722642366792e-5_f64) * t11623 + F::cast_from(0.11742981196020707897e-4_f64) * t11627 + F::cast_from(0.73393632475129424356e-6_f64) * t11630 + F::cast_from(0.43497959513593372169e-7_f64) * t11634 + F::cast_from(0.73393632475129424356e-6_f64) * t11638 + F::cast_from(0.11742981196020707897e-4_f64) * t11641 - F::cast_from(0.17098714139140853038e-6_f64) * t11644 - F::cast_from(0.49871249572494154694e-7_f64) * t11649 + F::cast_from(0.15388842725226767735e-5_f64) * t11651 + F::cast_from(0.46971924784082831588e-4_f64) * t11653;
    t11655
}
