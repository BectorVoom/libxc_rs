//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1958/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1958<F: Float>(t1936: F, t30138: F, t4248: F, t7741: F, t5920: F, t93: F, t7889: F, t1312: F, t30004: F, t18245: F, t1937: F, t7735: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30140 = F::cast_from(4.0_f64) * t30138 * t1936;
    let t30142 = F::cast_from(4.0_f64) * t4248 * t7741;
    let t30143 = t93 * t5920;
    let t30145 = F::cast_from(2.0_f64) * t30143 * t1936;
    let t30147 = F::cast_from(4.0_f64) * t7889 * t7741;
    let t30149 = F::cast_from(2.0_f64) * t1312 * t30004;
    let t30154 = F::cast_from(2.0_f64) * t18245 * t1937;
    let t30156 = F::cast_from(4.0_f64) * t30138 * t1937;
    let t30158 = F::cast_from(4.0_f64) * t4248 * t7735;
    (t30140, t30142, t30143, t30145, t30147, t30149, t30154, t30156, t30158)
}
