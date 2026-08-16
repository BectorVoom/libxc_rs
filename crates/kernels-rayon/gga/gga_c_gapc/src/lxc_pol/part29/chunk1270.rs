//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1270/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1270(t11258: f64, t2932: f64, t3946: f64, t1006: f64, t3639: f64, t4026: f64, t35568: f64, t583: f64, t8524: f64, t3635: f64, t8422: f64, t35628: f64, t35631: f64, t35634: f64, t35638: f64, t35640: f64, t35643: f64, t35647: f64, t35650: f64, t35653: f64) -> f64 {
    let t35656 = t2932 * t3946 * t11258;
    let t35659 = t1006 * t3639 * t4026;
    let t35662 = t8524 * t35568 * t583;
    let t35664 = t8422 * t3635;
    let t35666 = 0.86898242813537603824e-4_f64 * t35628 + 0.43449121406768801912e-4_f64 * t35631 + 0.43449121406768801912e-4_f64 * t35634 + 0.27155700879230501195e-5_f64 * t35638 + 0.5431140175846100239e-5_f64 * t35640 + 0.3218855744218122075e-6_f64 * t35643 - 0.23761238269326688546e-5_f64 * t35647 - 0.23761238269326688546e-5_f64 * t35650 + 0.22120729660314597581e-6_f64 * t35653 + 0.86898242813537603824e-4_f64 * t35656 - 0.86898242813537603824e-4_f64 * t35659 + 0.4049114220917933205e-4_f64 * t35662 + 0.14036929299182168444e-2_f64 * t35664;
    t35666
}
