//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1773/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1773(t112: f64, t7415: f64, t111: f64, t2169: f64, t191: f64, t192: f64, t5118: f64, t2020: f64, t6997: f64, t7685: f64, t1390: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24969 = t7415 * t112;
    let t24972 = t2169 * t111;
    let t24987 = t5118 * t191 * t192;
    let t24988 = t24987 * t2020;
    let t24989 = t7685 * t6997;
    let t24990 = t1390 * t5187;
    (t24969, t24972, t24987, t24988, t24989, t24990)
}
