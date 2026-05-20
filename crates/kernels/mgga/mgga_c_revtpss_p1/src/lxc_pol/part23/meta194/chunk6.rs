//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1175/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1175<F: Float>(t670: F, t93: F, t1312: F, t1518: F, t2322: F, t4246: F, t4248: F, t4292: F, t1450: F, t1907: F) -> (F, F, F) {
    let t5523 = t93 * t670;
    let t5528 = F::new(2.0) * t1312 * t4292 + F::new(2.0) * t1518 * t2322 + F::new(2.0) * t1518 * t5523 + F::new(2.0) * t4248 * t670 + t4246;
    let t5532 = t1907 * t1450;
    (t5523, t5528, t5532)
}
