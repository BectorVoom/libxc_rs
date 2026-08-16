//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 966/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk966<F: Float>(t31878: F, t3453: F, t309: F, t945: F, t1219: F, t615: F, t7911: F, t2137: F, t7930: F, t955: F, t7884: F, t7941: F) -> (F, F, F, F, F, F) {
    let t31879 = t31878 * t3453;
    let t31935 = t945 * t309;
    let t31965 = t615 * t7911 * t1219;
    let t32003 = t2137 * t7930;
    let t32010 = t955 * t309;
    let t32041 = t7884 * t7941;
    (t31879, t31935, t31965, t32003, t32010, t32041)
}
