//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2111/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2111(t16148: f64, t24995: f64, t8945: f64, t22949: f64, t7685: f64, t25010: f64, t6876: f64, t1307: f64, t19577: f64, t8643: f64, t1983: f64, t22584: f64, t26167: f64) -> (f64, f64, f64, f64, f64) {
    let t86679 = 12.0_f64 * t24995 * t8945 * t16148;
    let t86682 = t7685 * t22949;
    let t86684 = 2.0_f64 * t6876 * t25010;
    let t86685 = t19577 * t1307;
    let t86688 = 12.0_f64 * t24995 * t8643 * t86685;
    let t86693 = 3.0_f64 * t1983 * t26167 * t22584;
    (t86679, t86682, t86684, t86688, t86693)
}
