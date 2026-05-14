//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1097/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1097<F: Float>(t35628: F, t35631: F, t35634: F, t35638: F, t35640: F, t35643: F, t35647: F, t35650: F, t35653: F, t35656: F, t35659: F, t35662: F, t35664: F, t2911: F, t5918: F, t999: F) -> (F, F) {
    let t35666 = 0.86898242813537603824e-4 * t35628 + 0.43449121406768801912e-4 * t35631 + 0.43449121406768801912e-4 * t35634 + 0.27155700879230501195e-5 * t35638 + 0.5431140175846100239e-5 * t35640 + 0.3218855744218122075e-6 * t35643 - 0.23761238269326688546e-5 * t35647 - 0.23761238269326688546e-5 * t35650 + 0.22120729660314597581e-6 * t35653 + 0.86898242813537603824e-4 * t35656 - 0.86898242813537603824e-4 * t35659 + 0.4049114220917933205e-4 * t35662 + 0.14036929299182168444e-2 * t35664;
    let t35668 = t2911 * t999 * t5918;
    (t35666, t35668)
}
