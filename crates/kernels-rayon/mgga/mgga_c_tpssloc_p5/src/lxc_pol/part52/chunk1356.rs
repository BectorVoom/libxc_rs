//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1356/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1356(t1307: f64, t7752: f64, t22574: f64, t8643: f64, t33085: f64, t6535: f64, t22461: f64, t7461: f64, t26103: f64, t25980: f64, t6517: f64, t26179: f64, t8327: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120705 = t7752 * t1307;
    let t120708 = 6.0_f64 * t22574 * t8643 * t120705;
    let t120709 = t33085 * t6535;
    let t120711 = t22461 * t7461;
    let t120714 = t26103 * t7461;
    let t120716 = t6517 * t25980;
    let t120719 = 2.0_f64 * t26179 * t8327;
    (t120708, t120709, t120711, t120714, t120716, t120719)
}
