//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 968/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk968<F: Float>(t179: F, t2600: F, t3396: F, t10487: F, t10488: F, t10489: F, t10490: F, t10491: F, t10492: F, t4867: F, t4870: F, t4876: F, t4879: F, t4881: F, t4884: F) -> (F, F) {
    let t10586 = t179 * t2600 * t3396;
    let t10589 = t4867 + t4870 - t4876 - t4879 - t10487 + t10488 - t4881 - t10489 - t4884 + t10490 + t10491 + t10492;
    (t10586, t10589)
}
