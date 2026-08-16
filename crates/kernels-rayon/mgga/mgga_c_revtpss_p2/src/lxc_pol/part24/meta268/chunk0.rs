//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1040/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1040(t14613: f64, t162: f64, t5940: f64, t705: f64, t2411: f64, t6079: f64, t10446: f64, t5819: f64, t10457: f64, t5944: f64, t750: f64, t189: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18259 = t14613 * t162;
    let t18263 = t705 * t5940;
    let t18268 = t6079 * t2411;
    let t18272 = t10446 * t5819;
    let t18286 = t10457 * t5819;
    let t18301 = t5944 * t750;
    let t18305 = t189 * t5825;
    (t18259, t18263, t18268, t18272, t18286, t18301, t18305)
}
