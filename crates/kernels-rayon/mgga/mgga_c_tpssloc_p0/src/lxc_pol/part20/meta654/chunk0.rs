//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2419/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2419(t10750: f64, t13723: f64, t959: f64, t10757: f64, t1580: f64, t41825: f64, t10853: f64, t4483: f64, t13508: f64, t2940: f64, t10713: f64, t10756: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49502 = 0.14035736694323150897e2_f64 * t959 * t13723 * t10750;
    let t49506 = 0.12304822629859687989e5_f64 * t959 * t41825 * t1580 * t10757;
    let t49508 = 0.51947577317044391277e2_f64 * t4483 * t10853;
    let t49510 = 0.51947577317044391277e2_f64 * t2940 * t13508;
    let t49512 = 0.35089341735807877242e1_f64 * t4483 * t10713;
    let t49513 = t300 * t10756;
    (t49502, t49506, t49508, t49510, t49512, t49513)
}
