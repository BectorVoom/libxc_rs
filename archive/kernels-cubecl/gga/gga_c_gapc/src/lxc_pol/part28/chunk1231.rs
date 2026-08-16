//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1231/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1231<F: Float>(t11384: F, t26759: F, t26836: F, t11499: F, t1700: F, t633: F, t1040: F, t3687: F, t8863: F, t3115: F, t436: F, t8780: F) -> (F, F, F, F, F) {
    let t34565 = t11384 * t26759;
    let t34567 = t11384 * t26836;
    let t34570 = t633 * t11499 * t1700;
    let t34573 = t8863 * t3687 * t1040;
    let t34576 = t3115 * t436 * t8780;
    (t34565, t34567, t34570, t34573, t34576)
}
