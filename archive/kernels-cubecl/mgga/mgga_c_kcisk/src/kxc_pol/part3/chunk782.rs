//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 782/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk782<F: Float>(t1975: F, t5392: F, t1973: F, t5400: F, t1980: F, t4781: F, t4790: F, t1683: F, t12019: F, t1974: F, t1670: F, t4761: F) -> (F, F, F, F, F, F) {
    let t12066 = t1975 * t5392;
    let t12070 = t5392 * t5400 * t1973;
    let t12073 = t1980 * t4781;
    let t12076 = t4781 * t4790;
    let t12077 = t12076 * t1683;
    let t12081 = t12019 * t1974;
    let t12084 = t1670 * t4761;
    (t12066, t12070, t12073, t12077, t12081, t12084)
}
