//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2403/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2403(t12199: f64, t12208: f64, t3774: f64, t3862: f64, t241: f64, t6597: f64, t248: f64, t555: f64, t557: f64, t3787: f64, t3879: f64, t12019: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40425 = t12199 * t12208;
    let t40443 = t3774 * t3862;
    let t40445 = t6597 * t241;
    let t40449 = 13685.0_f64 / 31104.0_f64 * t555 * t40445 * t557 * t248;
    let t40486 = t3787 * t3879;
    let t40590 = 1.0_f64 / t12019 / t566;
    (t40425, t40443, t40445, t40449, t40486, t40590)
}
