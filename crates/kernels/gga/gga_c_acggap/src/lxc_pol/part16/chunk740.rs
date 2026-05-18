//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 740/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk740<F: Float>(t4210: F, t7932: F, t7942: F, t609: F, t862: F, t865: F, t2124: F, t310: F, t611: F, t848: F, t315: F, t7941: F) -> (F, F, F, F, F, F, F) {
    let t7943 = t7932 * t4210;
    let t7944 = t7942 * t7943;
    let t7948 = t862 * t609;
    let t7950 = F::new(0.13170898365871023197e1) * t7948 * t865;
    let t7957 = t310 * t2124;
    let t7962 = F::new(0.65854491829355115987e0) * t848 * t611;
    let t7963 = t315 * t7941;
    (t7943, t7944, t7948, t7950, t7957, t7962, t7963)
}
