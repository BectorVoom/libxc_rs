//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2413/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2413(t21238: f64, t2929: f64, t4497: f64, t959: f64, t17934: f64, t4489: f64, t4498: f64, t17565: f64, t21089: f64, t41825: f64, t17951: f64, t4483: f64) -> (f64, f64, f64, f64, f64) {
    let t68902 = t2929 * t21238;
    let t68905 = 0.17315859105681463759e2_f64 * t959 * t68902 * t4497;
    let t68910 = 0.35089341735807877242e1_f64 * t17934 * t4489;
    let t68912 = 0.51947577317044391276e2_f64 * t17934 * t4498;
    let t68916 = 0.12304822629859687989e5_f64 * t959 * t41825 * t21089 * t17565;
    let t68918 = 0.70178683471615754484e1_f64 * t4483 * t17951;
    (t68905, t68910, t68912, t68916, t68918)
}
