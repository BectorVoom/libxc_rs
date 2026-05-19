//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 680/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk680<F: Float>(t1335: F, t3862: F, t3861: F, t3793: F, t453: F, t3781: F, t1324: F, t3809: F, t1060: F, t250: F, t461: F, t1331: F, t659: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3863 = t3862 * t1335;
    let t3865 = F::new(2.0) * t3861 * t3863;
    let t3868 = F::cast_from(0.39862222222222222223e0_f64) * t3793;
    let t3873 = F::new(1.0)/F::sqrt(t453);
    let t3874 = t3873 * t3781;
    let t3876 = t1324 * t3809;
    let t3879 = t250 * t1060 * t461;
    let t3880 = F::cast_from(0.13692777777777777778e0_f64) * t3879;
    let t3881 = t659 * t1331;
    (t3863, t3865, t3868, t3873, t3874, t3876, t3879, t3880, t3881)
}
