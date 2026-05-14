//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1218/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1218<F: Float>(t12827: F, t12830: F, t12834: F, t12840: F, t12842: F, t12846: F, t12849: F, t18069: F, t18071: F, t18080: F, t18083: F, t18087: F, t4439: F, t12844: F, t6172: F, t531: F, t6183: F) -> (F, F, F) {
    let t18090 = -t12846 / 864.0 + t18069 / 324.0 + t4439 * t18071 / 96.0 + t12840 - t12827 / 1296.0 + t12830 / 1728.0 + t12834 / 1296.0 - t12849 / 864.0 + t12842 / 432.0 + t4439 * t18080 / 72.0 - t4439 * t18083 / 72.0 + t4439 * t18087 / 288.0;
    let t18091 = t12844 * t6172;
    let t18093 = t4439 * t18091 / 864.0;
    let t18094 = t6183 * t531;
    (t18090, t18093, t18094)
}
