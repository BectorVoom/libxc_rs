//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk857;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk858;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta156(t3377: f64, t3403: f64, t1129: f64, t1138: f64, t1148: f64, t1157: f64, t3258: f64, t3261: f64, t3268: f64, t3310: f64, t3318: f64, t3324: f64, t3327: f64, t3332: f64, t3334: f64, t3352: f64, t3357: f64, t3360: f64, t3369: f64, t3371: f64, t3376: f64, t3378: f64, t3396: f64, t3401: f64, t436: f64, t300: f64, t1143: f64, t1166: f64, t1156: f64, t3375: f64, t1164: f64, t1147: f64, t3395: f64, t3400: f64, t457: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3404, t3407) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk857(t3377, t3403, t1129, t1138, t1148, t1157, t3258, t3261, t3268, t3310, t3318, t3324, t3327, t3332, t3334, t3352, t3357, t3360, t3369, t3371, t3376, t3378, t3396, t3401, t436);
        let (t3408, t3410, t3411) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk858(t300, t3407, t3369, t1143);
        let (t3413, t3415, t3417, t3419, t3421, t3423, t3425, t3426) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk859(t1166, t3411, t1156, t3375, t3377, t1164, t1147, t3395, t3400, t3403, t457, t697);
    (t3404, t3408, t3410, t3411, t3413, t3415, t3417, t3419, t3421, t3423, t3425, t3426)
}
