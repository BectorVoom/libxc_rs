//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1000/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1000<F: Float>(t28449: F, t1967: F, t22044: F, t23104: F, t883: F, t2673: F, t7503: F, t23469: F, t9787: F, t2586: F, t2617: F, t7803: F, t7344: F, t948: F, t20671: F, t22543: F, t22980: F) -> (F, F, F, F, F, F, F) {
    let t28450 = 0.72851559312449424384e1 * t28449;
    let t28453 = t23104 * t1967 * t883 * t22044;
    let t28454 = 0.76685851907841499352e0 * t28453;
    let t28529 = 0.17875244975925213335e0 * t2673 * t7503;
    let t28563 = t23469 * t9787;
    let t28564 = 0.76685851907841499352e0 * t28563;
    let t28566 = t7803 * t2586 * t2617;
    let t28567 = 0.76685851907841499352e0 * t28566;
    let t28569 = t7803 * t948 * t7344;
    let t28570 = 0.38342925953920749676e0 * t28569;
    let t28585 = 0.17041300423964777634e0 * t22543 * t20671 * t22980;
    (t28450, t28454, t28529, t28564, t28567, t28570, t28585)
}
