//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1258/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1258<F: Float>(t13031: F, t18422: F, t18423: F, t18424: F, t18425: F, t18426: F, t18427: F, t20855: F, t23389: F, t2654: F, t4529: F, t6291: F, t6297: F, t6892: F, t6894: F, t7573: F, t8: F, t8521: F) -> (F,) {
    let t23392 = t6894 + 4.0 * t4529 + t2654 + t13031 - t18422 + t6291 - t18423 - t18424 + t8521 - t7573 - t18425 - t18426 - t6297 + t18427 + t6892 + t8 * (t20855 + t23389);
    (t23392,)
}
