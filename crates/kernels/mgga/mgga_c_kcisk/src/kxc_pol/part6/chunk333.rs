//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 333/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk333<F: Float>(t222: F, t227: F, t2063: F, t229: F, t2062: F, t44: F, t2059: F, t295: F, t442: F, zeta_threshold: F) -> (F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t2066 = piecewise3(t228, 0.0, 4.0 / 3.0 * t229 * t2063);
    let t2068 = (t2062 + t2066) * t44;
    let t2070 = piecewise3(t223, 0.0, t2059);
    let t2071 = t295 * t2070;
    let t2075 = t442 * t2059;
    (t2068, t2071, t2075)
}
