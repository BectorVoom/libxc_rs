//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1024/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1024(t23157: f64, t610: f64, t4425: f64, t7425: f64, t1599: f64, t4455: f64, t7492: f64, t1610: f64, t6176: f64, t1603: f64, t18142: f64, t18148: f64, t18152: f64, t18164: f64, t18170: f64, t18174: f64, t18178: f64, t18205: f64, t18213: f64, t23155: f64, t6141: f64, t6165: f64) -> (f64, f64) {
    let t23158 = t610 * t23157;
    let t23163 = t4425 * t7425;
    let t23164 = t1599 * t23163;
    let t23167 = t4455 * t7492;
    let t23168 = t23167 * t1610;
    let t23169 = t6176 * t23168;
    let t23172 = t18142 / 432.0_f64 - t18148 + t18152 - t23155 / 864.0_f64 + 11.0_f64 / 648.0_f64 * t23158 * t1603 + t6141 * t6165 / 54.0_f64 + t23164 / 1728.0_f64 - t18164 / 1296.0_f64 - t18170 - t18174 + t18178 - t18205 + t18213 + t1599 * t23169 / 96.0_f64;
    (t23158, t23172)
}
