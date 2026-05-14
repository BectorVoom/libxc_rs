//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 542/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk542<F: Float>(t2112: F, t2120: F, t2146: F, t307: F, t311: F, t786: F, t800: F, t803: F, t314: F) -> (F, F, F, F) {
    let t2149 = 0.65854491829355115987e0 * t2112 * t311 - 0.13170898365871023197e1 * t786 * t800 + 0.13170898365871023197e1 * t307 * t2120 - 0.65854491829355115987e0 * t307 * t2146;
    let t2153 = t803 * t803;
    let t2155 = t314 * t314;
    let t2156 = 1.0 / t2155;
    (t2149, t2153, t2155, t2156)
}
