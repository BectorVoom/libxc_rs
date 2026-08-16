//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1874;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta536<F: Float>(t5075: F, t7376: F, t7375: F, t225: F, t8034: F, t7364: F, t5072: F, t1215: F, t1409: F, t24851: F, t24589: F, t24812: F, t24827: F, t24849: F, t27406: F, t27481: F, t27484: F, t27492: F, t27498: F, t27502: F, t27507: F, t7283: F, t7368: F, t7373: F, t7378: F, t1755: F, t7327: F, t1090: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27510, t27511, t27516, t27517, t27520, t27521, t27525, t27526, t27529) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1874::<F>(t5075, t7376, t7375, t225, t8034, t7364, t5072, t1215, t1409, t24851, t24589, t24812, t24827, t24849, t27406, t27481, t27484, t27492, t27498, t27502, t27507, t7283, t7368, t7373, t7378);
        let (t27532, t27533, t27536) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1875::<F>(t1755, t7327, t1090, t7376, t8034);
    (t27510, t27511, t27516, t27517, t27520, t27521, t27525, t27526, t27529, t27532, t27533, t27536)
}
