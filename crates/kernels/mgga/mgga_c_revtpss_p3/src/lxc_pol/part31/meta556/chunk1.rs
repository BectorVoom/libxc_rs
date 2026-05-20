//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1964/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1964<F: Float>(t1936: F, t30138: F, t4248: F, t7741: F, t5920: F, t93: F, t7889: F, t1312: F, t30004: F, t1518: F, t28030: F, t29569: F, t29573: F, t30137: F, t6985: F) -> (F, F) {
    let t30140 = F::new(4.0) * t30138 * t1936;
    let t30142 = F::new(4.0) * t4248 * t7741;
    let t30143 = t93 * t5920;
    let t30145 = F::new(2.0) * t30143 * t1936;
    let t30147 = F::new(4.0) * t7889 * t7741;
    let t30149 = F::new(2.0) * t1312 * t30004;
    let t30150 = F::new(4.0) * t1518 * t28030 + F::new(2.0) * t5920 * t6985 + t29569 + F::new(2.0) * t29573 + t30137 + t30140 + t30142 + t30145 + t30147 + t30149;
    (t30143, t30150)
}
