//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2509/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2509<F: Float>(t14783: F, t699: F, t14786: F, t14789: F, t50946: F, t50948: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F) -> (F, F, F, F) {
    let t50968 = t699 * t14783;
    let t50970 = t699 * t14786;
    let t50972 = t699 * t14789;
    let t50974 = F::cast_from(0.72462e1_f64) * t50946 + F::cast_from(0.80513333333333333334e0_f64) * t50948 + F::cast_from(0.40256666666666666667e0_f64) * t50950 + F::cast_from(0.20128333333333333333e0_f64) * t50952 + F::cast_from(0.12077e1_f64) * t50954 - F::cast_from(0.60384999999999999999e0_f64) * t50957 - F::cast_from(0.60384999999999999999e0_f64) * t50961 - F::cast_from(0.36230999999999999999e1_f64) * t50966 + F::cast_from(0.11038e0_f64) * t50968 + F::cast_from(0.55190000000000000001e-1_f64) * t50970 + F::cast_from(0.33114000000000000001e0_f64) * t50972;
    (t50968, t50970, t50972, t50974)
}
