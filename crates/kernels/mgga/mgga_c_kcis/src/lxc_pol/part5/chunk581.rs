//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 581/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk581<F: Float>(t3820: F, t513: F, t1317: F, t1416: F, t3793: F, t1311: F, t1315: F, t1314: F, t465: F, t455: F, t453: F, t1060: F, t250: F, t461: F, t1331: F, t659: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3821 = t3820 * t513;
    let t3824 = t1317 * t1416;
    let t3833 = 0.55033333333333333333e-2 * t3793;
    let t3848 = 0.23744444444444444444e-1 * t3793;
    let t3856 = t1311 * t1315;
    let t3859 = t1314 * t465;
    let t3860 = 1.0 / t3859;
    let t3861 = t455 * t3860;
    let t3868 = 0.39862222222222222223e0 * t3793;
    let t3873 = 1.0/f64::sqrt(t453);
    let t3879 = t250 * t1060 * t461;
    let t3880 = 0.13692777777777777778e0 * t3879;
    let t3881 = t659 * t1331;
    (t3821, t3824, t3833, t3848, t3856, t3860, t3861, t3868, t3873, t3879, t3880, t3881)
}
