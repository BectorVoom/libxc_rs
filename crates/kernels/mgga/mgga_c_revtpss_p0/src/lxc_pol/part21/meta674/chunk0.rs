//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2475/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2475<F: Float>(t3494: F, t3519: F, t12258: F, t698: F, t13026: F, t240: F, t3361: F, t1146: F, t9303: F, t12270: F, t2304: F, t2439: F, t3424: F) -> (F, F, F, F, F, F, F, F) {
    let t43752 = F::new(1.0) / t3519 / t3494;
    let t43762 = t698 * t12258;
    let t43764 = t240 * t13026;
    let t43765 = t3361 * t3361;
    let t43766 = F::new(1.0) / t43765;
    let t43771 = t9303 * t1146;
    let t43773 = t698 * t12270;
    let t43776 = F::new(1.0) / t3361 / t2304;
    let t43781 = t2439 * t3424;
    (t43752, t43762, t43764, t43766, t43771, t43773, t43776, t43781)
}
