//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1145/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1145<F: Float>(t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F) -> (F, F) {
    let t14890 = F::cast_from(0.21908444444444444444e0_f64) * t14781;
    let t14911 = -F::cast_from(0.1898925e1_f64) * t14809 - F::cast_from(0.9494625e0_f64) * t14811 + F::cast_from(0.3071625e0_f64) * t14814 + F::cast_from(0.15358125e0_f64) * t14816 + F::cast_from(0.36514074074074074074e-1_f64) * t14818 + F::cast_from(0.26574814814814814816e0_f64) * t11137 + F::cast_from(0.66437037037037037038e-1_f64) * t11139 - F::cast_from(0.19931111111111111111e0_f64) * t11141 - F::cast_from(0.99655555555555555557e-1_f64) * t11143 + F::cast_from(0.3071625e0_f64) * t14824 + F::cast_from(0.33218518518518518518e0_f64) * t14728;
    (t14890, t14911)
}
