//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 775/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk775<F: Float>(t10491: F, t309: F, t2682: F, t312: F, t684: F, t1934: F, t824: F, t2875: F, t2874: F, t2360: F, t870: F, t10486: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10492 = t10491 * t309;
    let t10493 = t312 * t2682;
    let t10494 = t10493 * t684;
    let t10495 = t10492 * t10494;
    let t10498 = t1934 * t824;
    let t10499 = t2875 * t10498;
    let t10500 = t2874 * t10499;
    let t10503 = t870 * t2360;
    let t10504 = t10503 * t10486;
    (t10492, t10493, t10494, t10495, t10498, t10499, t10500, t10503, t10504)
}
