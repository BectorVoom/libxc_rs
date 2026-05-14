//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 619/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk619<F: Float>(t9240: F, t9257: F, t2666: F, t8973: F, t9017: F, t9021: F, t9023: F, t9025: F, t9027: F, t9031: F, t9033: F, t9037: F, t9039: F, t9041: F, t9044: F, t9048: F, t9052: F, t9056: F, t9059: F, t9063: F, t9067: F, t9070: F, t9073: F, t9080: F, t9083: F, t9087: F, t9091: F) -> (F, F, F, F) {
    let t9258 = t9240 + t9257;
    let t9262 = t2666 * t2666;
    let t9277 = 0.101171875e-1 * t8973 + 0.9375e-1 * t9017 - 0.20833333333333333333e-1 * t9021 + 0.20234375e-1 * t9023 - 0.5e0 * t9025 + 0.125e0 * t9027 - 0.9375e-1 * t9031 - 0.1875e0 * t9033 + 0.625e-1 * t9037 + 0.10791666666666666667e0 * t9039 - 0.26979166666666666666e-1 * t9041 + 0.5e0 * t9044;
    let t9290 = -0.125e0 * t9048 - 0.20234375e-1 * t9052 + 0.91666666666666666667e0 * t9056 - 0.33333333333333333334e0 * t9059 - 0.101171875e-1 * t9063 - 0.44965277777777777777e-2 * t9067 - 0.10791666666666666667e0 * t9070 + 0.26979166666666666666e-1 * t9073 - 0.34173611111111111111e0 * t9080 + 0.14388888888888888889e0 * t9083 - 0.13489583333333333333e-1 * t9087 + 0.1875e0 * t9091;
    (t9258, t9262, t9277, t9290)
}
