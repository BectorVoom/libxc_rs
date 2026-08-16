//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1427/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1427(t10547: f64, t6820: f64, t204: f64, t2476: f64, t34411: f64, t10388: f64, t10489: f64, t10492: f64, t1445: f64, t1555: f64, t1580: f64, t1617: f64, t1641: f64, t1646: f64, t31382: f64, t31386: f64, t31394: f64, t31502: f64, t31800: f64, t31829: f64, t3371: f64, t35172: f64, t35174: f64, t35178: f64, t35183: f64, t567: f64, t574: f64, t597: f64) -> f64 {
    let t35185 = 0.25025342966295298669e1_f64 * t10547 * t6820;
    let t35188 = 0.46011511144704899612e1_f64 * t2476 * t204 * t34411;
    let t35189 = -0.71500979903700853338e0_f64 * t1555 * t3371 * t1646 + 0.46011511144704899612e1_f64 * t567 * t1445 * t31502 + 0.46011511144704899612e1_f64 * t1617 * t10388 - 0.92023022289409799224e1_f64 * t1641 * t10489 + 0.23005755572352449806e2_f64 * t1580 * t10492 - 0.92023022289409799224e1_f64 * t574 * t1445 * t31800 + 0.23005755572352449806e2_f64 * t597 * t1445 * t31829 + t35172 - t35174 - t35178 - t31382 + t31386 + t31394 + t35183 - t35185 + t35188;
    t35189
}
