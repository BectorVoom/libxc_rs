//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1181/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1181(t40116: f64, t1445: f64, t1450: f64, t447: f64, t46919: f64, t13728: f64, t4614: f64, t597: f64, t41759: f64, t41761: f64, t41767: f64, t41769: f64, t41773: f64, t41777: f64, t41781: f64, t41783: f64, t41787: f64) -> f64 {
    let t47895 = 0.85206502119823888171e-1_f64 * t40116;
    let t47900 = 0.23005755572352449806e1_f64 * t1450 * t1445 * t46919 * t447;
    let t47902 = t597 * t4614 * t13728;
    let t47904 = -t47895 - t41759 + t41761 + t41767 - 0.92023022289409799224e1_f64 * t41769 - t41773 + t41777 + t41781 - t41783 - t41787 - t47900 + 0.15337170381568299871e2_f64 * t47902;
    t47904
}
