//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1731/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1731<F: Float>(t23110: F, t232: F, t236: F, t828: F, t23109: F, t6609: F, t838: F, t6589: F, t6597: F, t281: F, t22690: F, t776: F, t841: F) -> (F, F, F, F, F, F) {
    let t23113 = t23110 * t236 * t828 * t232;
    let t23114 = t23109 * t23113;
    let t23119 = t6609 * t838;
    let t23121 = t6597 * t6589;
    let t23122 = t23121 * t281;
    let t23124 = t22690 * t841 * t776;
    (t23113, t23114, t23119, t23121, t23122, t23124)
}
