//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1368;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta317<F: Float>(t10922: F, t973: F, t2960: F, t3139: F, t1030: F, t363: F, t3068: F, t1058: F, t3030: F, t990: F, t3032: F, t3129: F, t3038: F, t3087: F, t372: F, t364: F, t354: F, t1009: F, t3020: F, t1011: F, t1019: F, t1040: F, t3077: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10923, t10927, t10937, t10947, t10948, t10949) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1368::<F>(t10922, t973, t2960, t3139, t1030, t363, t3068, t1058, t3030, t990, t3032, t3129);
        let (t10952, t10957, t10960, t10962, t10965) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1369::<F>(t10948, t3038, t3087, t372, t364, t354, t1009, t3020, t1011, t1019, t1040, t3077);
    (t10923, t10927, t10937, t10947, t10949, t10952, t10957, t10960, t10962, t10965)
}
