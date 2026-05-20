//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2056/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2056<F: Float>(t26004: F, t5690: F, t13951: F, t2018: F, t807: F, t25240: F, t3964: F, t5617: F, t27857: F, t689: F, t25904: F, t786: F, t97961: F) -> (F, F, F, F, F, F) {
    let t98269 = t26004 * t5690;
    let t98270 = F::new(7.0) / F::new(72.0) * t98269;
    let t98281 = t807 * t2018 * t13951;
    let t98282 = F::cast_from(0.11433071498151929859e-3_f64) * t98281;
    let t98285 = t3964 * t25240 * t5617;
    let t98303 = t27857 * t689;
    let t98305 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t98303;
    let t98308 = t786 * t97961;
    (t98270, t98282, t98285, t98303, t98305, t98308)
}
