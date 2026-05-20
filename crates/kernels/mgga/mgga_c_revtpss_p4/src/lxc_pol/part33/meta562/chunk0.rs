//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1959/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1959<F: Float>(t2042: F, t6941: F, t1916: F, t7950: F, t7953: F, t1936: F, t5883: F, t572: F, t1518: F, t28276: F, t5920: F, t7330: F) -> (F, F, F, F, F, F, F, F) {
    let t30180 = F::new(3.0) * t6941 * t2042;
    let t30182 = F::new(12.0) * t1916 * t7950;
    let t30184 = F::new(6.0) * t1916 * t7953;
    let t30185 = t5883 * t1936;
    let t30187 = F::new(6.0) * t572 * t30185;
    let t30188 = t28276 * t1518;
    let t30190 = F::new(12.0) * t572 * t30188;
    let t30191 = t7330 * t5920;
    (t30180, t30182, t30184, t30185, t30187, t30188, t30190, t30191)
}
