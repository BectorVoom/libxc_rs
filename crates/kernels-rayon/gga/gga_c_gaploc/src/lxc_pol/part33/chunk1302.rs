//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1302/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1302(t34278: f64, t1415: f64, t7030: f64, t8330: f64, t2365: f64, t25740: f64, t7025: f64, t26126: f64, t544: f64, t21139: f64, t10513: f64, t18067: f64, t6964: f64) -> (f64, f64, f64, f64, f64) {
    let t34279 = 0.29792074959875355558e-1_f64 * t34278;
    let t34281 = t1415 * t8330 * t7030;
    let t34282 = 0.29792074959875355558e-1_f64 * t34281;
    let t34284 = t7025 * t2365 * t25740;
    let t34285 = 0.14896037479937677779e-1_f64 * t34284;
    let t34286 = t544 * t26126;
    let t34288 = 0.50050685932590597338e1_f64 * t34286 * t21139;
    let t34291 = 0.85801175884441024006e1_f64 * t18067 * t6964 * t10513;
    (t34279, t34282, t34285, t34288, t34291)
}
