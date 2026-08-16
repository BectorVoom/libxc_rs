//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk857;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk858;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta156<F: Float>(t3377: F, t3403: F, t1129: F, t1138: F, t1148: F, t1157: F, t3258: F, t3261: F, t3268: F, t3310: F, t3318: F, t3324: F, t3327: F, t3332: F, t3334: F, t3352: F, t3357: F, t3360: F, t3369: F, t3371: F, t3376: F, t3378: F, t3396: F, t3401: F, t436: F, t300: F, t1143: F, t1166: F, t1156: F, t3375: F, t1164: F, t1147: F, t3395: F, t3400: F, t457: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3404, t3407) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk857::<F>(t3377, t3403, t1129, t1138, t1148, t1157, t3258, t3261, t3268, t3310, t3318, t3324, t3327, t3332, t3334, t3352, t3357, t3360, t3369, t3371, t3376, t3378, t3396, t3401, t436);
        let (t3408, t3410, t3411) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk858::<F>(t300, t3407, t3369, t1143);
        let (t3413, t3415, t3417, t3419, t3421, t3423, t3425, t3426) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk859::<F>(t1166, t3411, t1156, t3375, t3377, t1164, t1147, t3395, t3400, t3403, t457, t697);
    (t3404, t3408, t3410, t3411, t3413, t3415, t3417, t3419, t3421, t3423, t3425, t3426)
}
