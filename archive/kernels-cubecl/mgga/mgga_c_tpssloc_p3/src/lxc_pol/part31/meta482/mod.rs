//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1645;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta482<F: Float>(t1352: F, t26421: F, t6976: F, t22633: F, t22705: F, t7736: F, t22704: F, t6883: F, t7741: F, t1998: F, t5318: F, t214: F, t1985: F, t7740: F, t794: F, t6897: F, t552: F, t6604: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26422, t26423, t26424, t26426, t26427, t26429, t26432, t26433) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1645::<F>(t1352, t26421, t6976, t22633, t22705, t7736, t22704, t6883, t7741, t1998, t5318, t214);
        let (t26434, t26436, t26437, t26446) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1646::<F>(t1985, t26433, t7740, t794, t6897, t552, t6604);
    (t26422, t26423, t26424, t26426, t26427, t26429, t26432, t26433, t26434, t26436, t26437, t26446)
}
