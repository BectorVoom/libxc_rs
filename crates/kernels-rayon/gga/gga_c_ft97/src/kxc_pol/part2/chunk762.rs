//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 762/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk762(t11960: f64, t488: f64, t83: f64, t1588: f64, t1871: f64, t986: f64, t10970: f64, t1651: f64, t447: f64, t1643: f64, t1866: f64, t3206: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11961 = t488 * t11960;
    let t11962 = t83 * t11961;
    let t11966 = t1871 * t986 * t1588;
    let t11969 = t83 * t10970;
    let t11973 = t447 * t986 * t1651;
    let t11977 = t1866 * t986 * t1643;
    let t11981 = 2.0_f64 / 27.0_f64 * t8392 * t3206;
    (t11961, t11962, t11966, t11969, t11973, t11977, t11981)
}
