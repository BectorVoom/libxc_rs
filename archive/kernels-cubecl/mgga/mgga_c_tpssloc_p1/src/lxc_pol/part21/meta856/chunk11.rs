//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3107/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3107<F: Float>(t43777: F, t43855: F, t43859: F, t43861: F, t43863: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t50952: F, t50954: F) -> F {
    let t64374 = t43777 - F::cast_from(0.30661111111111111111e-1_f64) * t43855 - F::cast_from(0.49057777777777777778e0_f64) * t43859 + F::cast_from(0.91983333333333333333e-1_f64) * t43861 + F::cast_from(0.18396666666666666667e0_f64) * t43863 - F::cast_from(0.80513333333333333336e0_f64) * t50903 - F::cast_from(0.40256666666666666668e0_f64) * t50905 - F::cast_from(0.12077e1_f64) * t50907 - F::cast_from(0.35783703703703703705e0_f64) * t50919 - F::cast_from(0.22364814814814814815e0_f64) * t50921 + F::cast_from(0.10735111111111111112e1_f64) * t50948 + F::cast_from(0.26837777777777777778e0_f64) * t50950 + F::cast_from(0.13418888888888888889e0_f64) * t50952 + F::cast_from(0.80513333333333333335e0_f64) * t50954;
    t64374
}
