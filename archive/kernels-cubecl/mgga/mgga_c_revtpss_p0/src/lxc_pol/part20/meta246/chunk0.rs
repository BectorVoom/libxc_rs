//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1071/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1071<F: Float>(t11173: F, t996: F, t1096: F, t3325: F, t3269: F, t3075: F, t1079: F, t1071: F, t3057: F, t3259: F, t994: F, t342: F) -> (F, F, F, F, F, F, F) {
    let t11174 = t996 * t11173;
    let t11177 = t1096 * t3325;
    let t11178 = t3269 * t11177;
    let t11183 = t3075 * t1096;
    let t11184 = t1079 * t11183;
    let t11187 = t3057 * t1071;
    let t11190 = t994 * t3259;
    let t11195 = t342 * t3259;
    (t11174, t11177, t11178, t11184, t11187, t11190, t11195)
}
