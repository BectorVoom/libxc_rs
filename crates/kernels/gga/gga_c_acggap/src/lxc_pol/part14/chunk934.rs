//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 934/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk934<F: Float>(t2131: F, t2147: F, t463: F, t7997: F, t157: F, t406: F, t847: F, t7932: F, t7963: F, t2132: F, t2139: F, t7885: F, t879: F) -> (F, F, F) {
    let t32187 = t2131 * t2147 * t7997 * t463;
    let t32194 = t847 * t406 * t157;
    let t32196 = t7963 * t7932 * t32194;
    let t32210 = F::cast_from(0.78062653693846795158e1_f64) * t7885 * t2132 * t2139 * t879;
    (t32187, t32196, t32210)
}
