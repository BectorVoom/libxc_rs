//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 863/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk863(t487: f64, t7763: f64, t100: f64, t38477: f64, t1786: f64, t1852: f64, t488: f64, t8216: f64, t8326: f64, t38463: f64, t38052: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39021 = t487 * t7763;
    let t39026 = t38477 * t100;
    let t39107 = t1786 * t1852;
    let t39120 = t8216 * t488;
    let t39167 = t8326 * t488;
    let t39230 = t38463 * t100;
    let t39243 = t38052 * t82;
    (t39021, t39026, t39107, t39120, t39167, t39230, t39243)
}
