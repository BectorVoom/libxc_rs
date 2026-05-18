//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 795/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk795<F: Float>(t10763: F, t2843: F, t840: F, t2894: F, t824: F, t1882: F, t2803: F, t8232: F, t842: F, t10246: F, t10259: F, t10265: F, t10269: F, t10273: F, t10276: F, t10279: F, t10282: F, t10391: F, t10394: F, t10400: F, t10624: F, t10634: F) -> (F, F, F, F, F) {
    let t10765 = t840 * t2843 * t10763;
    let t10769 = t840 * t2894 * t824;
    let t10771 = t1882 * t2803;
    let t10773 = t8232 * t842;
    let t10786 = -t10391 + t10394 - F::new(4.0) / F::new(3.0) * t10400 - F::new(6.0) * t10265 - F::new(2.0) * t10276 + t10624 / F::new(2.0) + F::new(3.0) / F::new(8.0) * t10634 - F::new(2.0) / F::new(3.0) * t10246 - t10259 / F::new(3.0) + F::new(6.0) * t10269 - F::new(10.0) / F::new(27.0) * t10273 - F::new(4.0) / F::new(9.0) * t10279 + t10282 / F::new(3.0);
    (t10765, t10769, t10771, t10773, t10786)
}
