//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2068;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta626<F: Float>(t3540: F, t7334: F, t11832: F, t2127: F, t2132: F, t2136: F, t2250: F, t24684: F, t7324: F, t7331: F, t23413: F, t461: F, t11745: F, t24729: F, t24746: F, t86192: F, t10401: F, t24739: F, t3610: F, t3624: F, t24740: F, t3604: F, t11838: F, t7310: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86275, t86278, t86282, t86292, t86293, t86296) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2068::<F>(t3540, t7334, t11832, t2127, t2132, t2136, t2250, t24684, t7324, t7331, t23413, t461);
        let (t86299, t86313, t86324, t86327, t86330, t86341) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2069::<F>(t11745, t24729, t2132, t24746, t86192, t10401, t24739, t3610, t3624, t24740, t3604, t11838, t7310);
    (t86275, t86278, t86282, t86292, t86293, t86296, t86299, t86313, t86324, t86327, t86330, t86341)
}
