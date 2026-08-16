//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1363/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1363(t34284: f64, t26126: f64, t544: f64, t21139: f64, t10513: f64, t18067: f64, t6964: f64, t10524: f64, t10527: f64, t1397: f64, t10314: f64, t2476: f64, t580: f64) -> (f64, f64, f64, f64, f64) {
    let t34285 = 0.14896037479937677779e-1_f64 * t34284;
    let t34286 = t544 * t26126;
    let t34288 = 0.50050685932590597338e1_f64 * t34286 * t21139;
    let t34291 = 0.85801175884441024006e1_f64 * t18067 * t6964 * t10513;
    let t34294 = 0.42900587942220512002e1_f64 * t1397 * t10524 * t10527;
    let t34297 = 0.12269736305254639897e2_f64 * t2476 * t580 * t10314;
    (t34285, t34288, t34291, t34294, t34297)
}
