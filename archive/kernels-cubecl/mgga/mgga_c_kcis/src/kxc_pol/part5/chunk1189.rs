//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1189/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1189<F: Float>(t1184: F, t6728: F, t10753: F, t6720: F, t14875: F, t1801: F, t13321: F, t3436: F, t5177: F, t19540: F, t355: F, t381: F) -> (F, F, F, F, F) {
    let t19895 = t1184 * t6728;
    let t19897 = t10753 * t6720;
    let t19899 = t14875 * t1801;
    let t19901 = t13321 * t3436;
    let t19902 = t19901 * t5177;
    let t19904 = t19540 * t355;
    let t19905 = t19904 * t381;
    (t19895, t19897, t19899, t19902, t19905)
}
