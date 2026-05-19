//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1267/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1267<F: Float>(t11258: F, t2932: F, t3946: F, t1006: F, t3639: F, t4026: F, t35568: F, t583: F, t8524: F, t3635: F, t8422: F, t35628: F, t35631: F, t35634: F, t35638: F, t35640: F, t35643: F, t35647: F, t35650: F, t35653: F) -> F {
    let t35656 = t2932 * t3946 * t11258;
    let t35659 = t1006 * t3639 * t4026;
    let t35662 = t8524 * t35568 * t583;
    let t35664 = t8422 * t3635;
    let t35666 = F::cast_from(0.86898242813537603824e-4_f64) * t35628 + F::cast_from(0.43449121406768801912e-4_f64) * t35631 + F::cast_from(0.43449121406768801912e-4_f64) * t35634 + F::cast_from(0.27155700879230501195e-5_f64) * t35638 + F::cast_from(0.5431140175846100239e-5_f64) * t35640 + F::cast_from(0.3218855744218122075e-6_f64) * t35643 - F::cast_from(0.23761238269326688546e-5_f64) * t35647 - F::cast_from(0.23761238269326688546e-5_f64) * t35650 + F::cast_from(0.22120729660314597581e-6_f64) * t35653 + F::cast_from(0.86898242813537603824e-4_f64) * t35656 - F::cast_from(0.86898242813537603824e-4_f64) * t35659 + F::cast_from(0.4049114220917933205e-4_f64) * t35662 + F::cast_from(0.14036929299182168444e-2_f64) * t35664;
    t35666
}
