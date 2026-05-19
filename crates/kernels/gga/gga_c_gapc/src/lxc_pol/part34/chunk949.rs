//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 949/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk949<F: Float>(t9898: F, t9901: F, t9904: F, t9908: F, t9910: F, t9914: F, t9917: F, t9924: F, t9930: F, t9935: F, t9937: F, t9939: F, t9941: F) -> F {
    let t10991 = F::cast_from(0.25781643416302550011e-8_f64) * t9898 + F::cast_from(0.42270452978984302532e-6_f64) * t9901 + F::cast_from(0.12380169846338434109e-5_f64) * t9904 - F::cast_from(0.84410248952307505288e-7_f64) * t9908 - F::cast_from(0.16882049790461501058e-6_f64) * t9910 - F::cast_from(0.84410248952307505288e-7_f64) * t9914 - F::cast_from(0.10005428175813516294e-7_f64) * t9917 + F::cast_from(0.20010856351627032588e-7_f64) * t9924 - F::cast_from(0.14591249423061377928e-8_f64) * t9930 + F::cast_from(0.49239311888846044752e-7_f64) * t9935 + F::cast_from(0.21642471925239962898e-3_f64) * t9937 + F::cast_from(0.2318836277704281739e-4_f64) * t9939 + F::cast_from(0.80043425406508130349e-7_f64) * t9941;
    t10991
}
