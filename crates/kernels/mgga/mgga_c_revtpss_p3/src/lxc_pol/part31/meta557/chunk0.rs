//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1966/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1966<F: Float>(t29991: F, t30159: F, t3: F, t2042: F, t6941: F, t1916: F, t7950: F, t7953: F, t1936: F, t5883: F, t572: F, t1518: F, t28276: F, param_d: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30160 = t29991 + t30159;
    let t30161 = t3 * t30160;
    let t30171 = param_d * t30160;
    let t30180 = F::new(3.0) * t6941 * t2042;
    let t30182 = F::new(12.0) * t1916 * t7950;
    let t30184 = F::new(6.0) * t1916 * t7953;
    let t30185 = t5883 * t1936;
    let t30187 = F::new(6.0) * t572 * t30185;
    let t30188 = t28276 * t1518;
    (t30160, t30161, t30171, t30180, t30182, t30184, t30185, t30187, t30188)
}
