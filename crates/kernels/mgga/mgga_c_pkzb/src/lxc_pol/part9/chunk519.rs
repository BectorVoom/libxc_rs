//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 519/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk519<F: Float>(t2145: F, t790: F, t2112: F, t2120: F, t307: F, t311: F, t786: F, t800: F) -> (F, F) {
    let t2146 = t790 * t2145;
    let t2149 = F::new(0.65854491829355115987e0) * t2112 * t311 - F::new(0.13170898365871023197e1) * t786 * t800 + F::new(0.13170898365871023197e1) * t307 * t2120 - F::new(0.65854491829355115987e0) * t307 * t2146;
    (t2146, t2149)
}
