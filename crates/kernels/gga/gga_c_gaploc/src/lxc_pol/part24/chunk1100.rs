//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1100/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1100<F: Float>(t22909: F, t9820: F, t2624: F, t7419: F, t9800: F, t1967: F, t22044: F, t23104: F, t883: F, t2673: F, t7503: F, t23469: F, t9787: F) -> (F, F, F, F, F) {
    let t28443 = F::new(0.29792074959875355558e-1) * t9820 * t22909;
    let t28449 = t9800 * t2624 * t7419;
    let t28450 = F::new(0.72851559312449424384e1) * t28449;
    let t28453 = t23104 * t1967 * t883 * t22044;
    let t28454 = F::new(0.76685851907841499352e0) * t28453;
    let t28529 = F::new(0.17875244975925213335e0) * t2673 * t7503;
    let t28563 = t23469 * t9787;
    (t28443, t28450, t28454, t28529, t28563)
}
