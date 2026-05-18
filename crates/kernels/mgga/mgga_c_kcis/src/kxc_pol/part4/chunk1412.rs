//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1412/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1412<F: Float>(t17307: F, t17310: F, t17313: F, t17314: F, t17315: F, t17317: F, t17319: F, t17322: F, t17325: F, t17328: F, t17709: F, t18354: F, t18364: F, t187: F) -> F {
    let t18367 = t17307 - t17310 + t17313 - t17314 - t17315 + t17317 - t17319 - t17322 + t17325 + t17328 - t17709 + t187 * (t18354 + t18364);
    t18367
}
