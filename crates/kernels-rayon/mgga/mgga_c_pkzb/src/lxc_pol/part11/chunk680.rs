//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 680/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk680(t2435: f64, t3913: f64, t133: f64, t3880: f64, t945: f64, t2447: f64, t1250: f64, t2433: f64, t2446: f64, t3273: f64, t3903: f64, t397: f64, t943: f64) -> (f64, f64, f64, f64, f64) {
    let t3914 = t3913 * t2435;
    let t3919 = t3880 * t133;
    let t3920 = t3919 * t945;
    let t3923 = t3913 * t2447;
    let t3928 = 0.13170898365871023197e1_f64 * t2433 * t3914 + 0.13170898365871023197e1_f64 * t3273 * t1250 + 0.65854491829355115987e0_f64 * t943 * t3920 - 0.65854491829355115987e0_f64 * t2446 * t3923 + 0.65854491829355115987e0_f64 * t397 * t3903;
    (t3914, t3919, t3920, t3923, t3928)
}
