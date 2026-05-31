//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1200/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1200<F: Float>(t23682: F, t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23660: F) -> F {
    let t24776 = F::cast_from(0.17757530864197530864e0_f64) * t23682;
    let t24788 = t24776 - F::cast_from(0.45662222222222222221e-1_f64) * t23620 - F::cast_from(0.3044148148148148148e-1_f64) * t23622 + F::cast_from(0.22831111111111111111e-1_f64) * t23624 + F::cast_from(0.25367901234567901233e-1_f64) * t23626 - F::cast_from(0.50735802469135802467e-1_f64) * t23630 - F::cast_from(0.17123333333333333333e-1_f64) * t23633 + F::cast_from(0.71030123456790123454e-1_f64) * t23635 - F::cast_from(0.9132444444444444444e-1_f64) * t23637 + F::cast_from(0.2283111111111111111e0_f64) * t23640 + F::cast_from(0.10274e0_f64) * t23644 + F::cast_from(0.13698666666666666667e0_f64) * t23660;
    t24788
}
