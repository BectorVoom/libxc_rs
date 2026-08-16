//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 962/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk962(t17376: f64, t17425: f64, t17483: f64, t17706: f64, t1506: f64, t1628: f64, t6220: f64, t2128: f64, t4481: f64, t4314: f64, t6188: f64, t1615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17708 = t17376 + t17425 + t17483 + t17706;
    let t17709 = t1506 * t17708;
    let t17710 = t6220 * t1628;
    let t17713 = t2128 * t4481;
    let t17730 = t6188 * t4314;
    let t17731 = t17730 * t1615;
    (t17708, t17709, t17710, t17713, t17730, t17731)
}
