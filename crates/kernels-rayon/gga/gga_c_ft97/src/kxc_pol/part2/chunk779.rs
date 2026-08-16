//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 779/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk779(t11171: f64, t11169: f64, t11204: f64, t528: f64, t8690: f64, t929: f64, t2007: f64, t3056: f64, t11180: f64, t11186: f64, t11189: f64, t11195: f64, t11198: f64, t1595: f64, t1655: f64, t3359: f64, t383: f64) -> (f64, f64, f64, f64) {
    let t12216 = 0.19257444444444444444e0_f64 * t11171;
    let t12217 = 0.6419148148148148148e-1_f64 * t11169;
    let t12223 = t528 * t11204;
    let t12225 = t8690 * t929;
    let t12228 = t2007 * t3056;
    let t12233 = -t12216 + t12217 - 0.9628722222222222222e-1_f64 * t11189 - 0.1604787037037037037e0_f64 * t11180 - 0.38514888888888888888e0_f64 * t11186 + 0.28886166666666666666e0_f64 * t11198 + 0.11554466666666666666e1_f64 * t11195 + 0.234754e0_f64 * t12223 + 0.1760655e0_f64 * t12225 * t1595 - 0.234754e0_f64 * t12228 * t383 - 0.117377e0_f64 * t3359 * t1655;
    (t12223, t12225, t12228, t12233)
}
