//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1357/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1357(t10868: f64, t820: f64, t843: f64, t10874: f64, t2482: f64, t27: f64, t10872: f64, t221: f64, t2485: f64, t10832: f64, t10845: f64, t823: f64, t9948: f64) -> (f64, f64, f64, f64) {
    let t40348 = t820 * t10868 * t843;
    let t40349 = t40348 * t10874;
    let t40352 = t2482 * t10868 * t27;
    let t40355 = t40352 * t2485 * t221 * t10872;
    let t40357 = t10845 * t10832;
    let t40360 = t820 * t823 * t9948;
    (t40349, t40355, t40357, t40360)
}
