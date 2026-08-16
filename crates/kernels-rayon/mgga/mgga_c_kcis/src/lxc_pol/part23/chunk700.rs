//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 700/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk700(t1629: f64, t1636: f64, t187: f64, t2268: f64, t4475: f64, t4480: f64, t633: f64, t7939: f64, t7941: f64, t7942: f64, t7945: f64, t7963: f64, t7996: f64, t7998: f64, t8001: f64, t8010: f64) -> f64 {
    let t8014 = t7939 - t7941 - t7942 + t7945 - t7963 + t187 * (-t1629 * t8010 - t1636 * t7998 - t2268 * t4475 + 2.0_f64 * t4480 * t8001 + t633 * t7996 - t7939 + t7941 + t7942 - t7945 + t7963);
    t8014
}
