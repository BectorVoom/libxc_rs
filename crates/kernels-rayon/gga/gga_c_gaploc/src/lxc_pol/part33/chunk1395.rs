//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1395/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1395(t38276: f64, t447: f64, t12001: f64, t12135: f64, t1445: f64, t1450: f64, t1589: f64, t204: f64, t2476: f64, t30920: f64, t34773: f64, t34774: f64, t34775: f64, t34776: f64, t34783: f64, t34790: f64, t34794: f64, t34797: f64, t34801: f64, t38392: f64, t38393: f64, t4771: f64, t557: f64, t597: f64) -> (f64, f64) {
    let t38731 = t38276 * t447;
    let t38738 = -0.46011511144704899612e1_f64 * t4771 * t12135 - 0.46011511144704899612e1_f64 * t1450 * t1445 * t38392 * t447 + 0.23005755572352449806e2_f64 * t597 * t1445 * t38393 - t34773 - t34774 + t30920 - t34775 - t34776 + 0.92023022289409799224e1_f64 * t2476 * t204 * t38731 + t34783 + t34790 - t34794 - t34797 - t34801 - 0.47667319935800568892e0_f64 * t557 * t1589 * t12001;
    (t38731, t38738)
}
