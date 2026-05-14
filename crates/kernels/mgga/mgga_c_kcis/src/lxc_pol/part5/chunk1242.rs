//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1242/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1242<F: Float>(t1591: F, t7490: F, t18059: F, t18060: F, t21933: F, t21937: F, t21941: F, t21945: F, t21949: F, t21958: F, t21961: F, t21963: F, t21965: F, t18079: F, t21110: F, t1610: F, t6944: F) -> (F, F, F, F) {
    let t23036 = t7490 * t1591;
    let t23052 = -0.19345601851851851852e-2 * t21933 + 0.12897067901234567901e-2 * t21937 - 0.11607361111111111111e-1 * t21941 + 0.51588271604938271605e-2 * t21945 - 0.77382407407407407408e-2 * t21949 - t18059 + t18060 - 0.17411041666666666666e-2 * t21958 + 0.11607361111111111111e-2 * t21961 - 0.25794135802469135802e-3 * t21963 + 0.23214722222222222221e-2 * t21965;
    let t23073 = t18079 * t21110;
    let t23076 = t6944 * t1610;
    (t23036, t23052, t23073, t23076)
}
