//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 931/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk931<F: Float>(t1286: F, t32405: F, t376: F, t32395: F, t1637: F, t7167: F, t23089: F, t7162: F, t32001: F, t32386: F, t1307: F, t5748: F) -> (F, F, F, F, F, F, F) {
    let t136041 = t1286 * t376 * t32405;
    let t136044 = t1286 * t376 * t32395;
    let t136058 = F::new(4.0) / F::new(27.0) * t1286 * t1637 * t7167;
    let t136059 = t7162 * t23089;
    let t136072 = t1286 * t376 * t32001;
    let t136075 = t1286 * t376 * t32386;
    let t136077 = t1307 * t5748;
    (t136041, t136044, t136058, t136059, t136072, t136075, t136077)
}
