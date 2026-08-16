//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1103/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1103(t28550: f64, t3984: f64, t2237: f64, t27483: f64, t27486: f64, t28501: f64, t28506: f64, t28508: f64, t28511: f64, t28514: f64, t28517: f64, t28520: f64, t28522: f64, t28526: f64, t28529: f64, t28532: f64, t28535: f64, t28544: f64, t28547: f64, t7895: f64, t7901: f64, t7908: f64, t7916: f64, t8151: f64, t8159: f64) -> (f64, f64) {
    let t28551 = t3984 * t28550;
    let t28554 = 0.16581944444444444444e-2_f64 * t28501 + 0.49745833333333333332e-2_f64 * t28506 - 0.44218518518518518517e-2_f64 * t28508 + 0.11054629629629629629e-2_f64 * t28511 - 0.33163888888888888888e-2_f64 * t28514 + 0.27636574074074074073e-2_f64 * t28517 - 0.16581944444444444444e-2_f64 * t28520 - t27483 + t27486 + 0.23168402777777777778e-3_f64 * t28522 - 0.24872916666666666666e-2_f64 * t28526 + 0.16581944444444444444e-2_f64 * t28529 - 0.24872916666666666666e-2_f64 * t28532 + 0.69505208333333333333e-3_f64 * t2237 * t28535 + 0.69505208333333333333e-3_f64 * t7895 * t8159 - 0.18534722222222222222e-2_f64 * t8151 * t7916 - 0.18534722222222222222e-2_f64 * t8151 * t7901 - 0.24734586805555555555e-3_f64 * t28544 * t7901 - 0.16581944444444444444e-2_f64 * t28547 + 0.23168402777777777778e-3_f64 * t7908 * t28551;
    (t28551, t28554)
}
