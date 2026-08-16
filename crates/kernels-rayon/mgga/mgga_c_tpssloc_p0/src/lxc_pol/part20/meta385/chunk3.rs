//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1755/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1755(t13213: f64, t13268: f64, t13331: f64, t13375: f64, t218: f64, t1509: f64, t852: f64) -> (f64, f64, f64) {
    let t13377 = t13213 + t13268 + t13331 + t13375;
    let t13378 = t218 * t13377;
    let t13380 = t852 * t1509;
    (t13377, t13378, t13380)
}
