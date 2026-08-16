//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1055;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta211(t1088: f64, t4733: f64, t123: f64, t3237: f64, t3238: f64, t4721: f64, t4726: f64, t4731: f64, t423: f64, t1098: f64, t1657: f64, t1119: f64, t1671: f64, t3259: f64, t1117: f64, t3264: f64, t1661: f64, t3270: f64, t1102: f64, t3274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4734, t4735, t4737, t4739, t4740, t4742) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1055(t1088, t4733, t123, t3237, t3238, t4721, t4726, t4731, t423, t1098, t1657, t1119);
        let (t4744, t4745, t4747, t4748, t4749, t4756) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1056(t1671, t3259, t1117, t3264, t1661, t3270, t1102, t3238, t3274, t4721, t4726, t4731, t4735);
    (t4734, t4735, t4737, t4739, t4740, t4742, t4744, t4745, t4747, t4748, t4749, t4756)
}
