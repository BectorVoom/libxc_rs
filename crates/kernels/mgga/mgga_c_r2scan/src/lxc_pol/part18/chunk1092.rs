//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1092/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1092<F: Float>(t39420: F, t25397: F, t37945: F, t38031: F, t10710: F, t10768: F, t25737: F, t25499: F, t37586: F, t25503: F, t37658: F, t11816: F, t37880: F) -> (F, F, F, F, F, F) {
    let t39421 = F::cast_from(0.25610080155860322884e0_f64) * t39420;
    let t39429 = t38031 * t37945 * t25397;
    let t39437 = t10768 * t10710 * t25737;
    let t39438 = F::cast_from(0.47609969197673950972e-2_f64) * t39437;
    let t39440 = t37586 * t10710 * t25499;
    let t39443 = t37658 * t10710 * t25503;
    let t39444 = F::cast_from(0.14282990759302185292e-1_f64) * t39443;
    let t39445 = t37880 * t11816;
    (t39421, t39429, t39438, t39440, t39444, t39445)
}
