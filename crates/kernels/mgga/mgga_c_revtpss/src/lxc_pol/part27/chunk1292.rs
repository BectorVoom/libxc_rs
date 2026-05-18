//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1292/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1292<F: Float>(t1937: F, t46126: F, t49851: F, t10416: F, t6993: F, t25081: F, t7234: F, t25083: F, t2014: F, t25089: F, t25190: F, t28167: F, t49616: F, t8717: F) -> (F, F, F, F, F, F) {
    let t95083 = F::new(2.0) * t46126 * t1937;
    let t95085 = F::new(6.0) * t49851 * t1937;
    let t95087 = F::new(6.0) * t10416 * t6993;
    let t95088 = t7234 * t25081;
    let t95090 = F::new(18.0) * t95088 * t25083;
    let t95096 = F::new(9.0) * t2014 * t25190 * t25089;
    let t95104 = F::new(18.0) * t28167 * t8717 * t49616;
    (t95083, t95085, t95087, t95090, t95096, t95104)
}
