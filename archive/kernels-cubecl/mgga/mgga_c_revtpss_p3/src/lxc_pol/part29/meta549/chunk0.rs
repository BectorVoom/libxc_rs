//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1886/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1886<F: Float>(t7289: F, t96282: F, t26277: F, t94776: F, t25950: F, t26292: F, t26230: F, t94764: F, t94768: F, t94763: F, t26234: F, t94890: F) -> (F, F, F, F, F, F) {
    let t96284 = F::cast_from(0.39982213492741449076e-1_f64) * t7289 * t96282;
    let t96287 = t94776 * t26277;
    let t96289 = t25950 * t26292;
    let t96291 = t26230 * t94764;
    let t96292 = t94768 * t96291;
    let t96294 = t94763 * t96291;
    let t96296 = t94890 * t26234;
    (t96284, t96287, t96289, t96292, t96294, t96296)
}
