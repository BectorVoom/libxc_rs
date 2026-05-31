//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1204/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1204<F: Float>(t23682: F, t23685: F, t23660: F, t23664: F, t23667: F, t23670: F, t23673: F, t23676: F, t23679: F, t23928: F, t23931: F, t23933: F, t23936: F, t23938: F) -> F {
    let t24863 = F::cast_from(0.5356037037037037037e1_f64) * t23682;
    let t24864 = F::cast_from(0.16979925925925925926e1_f64) * t23685;
    let t24870 = F::cast_from(0.41318e1_f64) * t23660 - F::cast_from(0.375102e1_f64) * t23664 + F::cast_from(0.83356e0_f64) * t23667 + F::cast_from(0.123954e2_f64) * t23670 - F::cast_from(0.13772666666666666667e1_f64) * t23673 - F::cast_from(0.34431666666666666667e1_f64) * t23676 - F::cast_from(0.185931e2_f64) * t23679 + t24863 + t24864 + F::cast_from(0.3529725e1_f64) * t23928 + F::cast_from(0.158837625e2_f64) * t23931 - F::cast_from(0.705945e1_f64) * t23933 - F::cast_from(0.94674375e0_f64) * t23936 + F::cast_from(0.1262325e1_f64) * t23938;
    t24870
}
