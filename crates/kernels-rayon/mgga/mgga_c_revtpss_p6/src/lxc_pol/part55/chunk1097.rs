//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1097/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1097(t2022: f64, t8085: f64, t8707: f64, t2097: f64, t7910: f64, t1892: f64, t3140: f64, t8477: f64, t1903: f64, t8708: f64, t32250: f64, t32690: f64, t32723: f64, t32724: f64, t32725: f64, t32731: f64, t33927: f64, t33965: f64, t33971: f64, t7921: f64, t7926: f64, t8706: f64, t8709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34222 = t8085 * t2022;
    let t34223 = t8707 * t34222;
    let t34226 = t2097 * t7910;
    let t34227 = t8707 * t34226;
    let t34230 = t1892 * t3140;
    let t34231 = t8477 * t34230;
    let t34236 = t8708 * t1903;
    let t34237 = t32250 * t34236;
    let t34240 = -t32723 + t32724 - t32725 + 0.7437465841810202164e-3_f64 * t33965 + 0.14874931683620404328e-2_f64 * t33971 + t32731 - 0.225875734067843736e-2_f64 * t33927 + 0.17347256376410398924e1_f64 * t32690 * t7921 + 0.57119737665102352616e0_f64 * t8706 * t34223 + 0.57119737665102352616e0_f64 * t8706 * t34227 + 0.57119737665102352616e0_f64 * t34231 * t8709 + 0.8673628188205199462e0_f64 * t32690 * t7926 - 0.17135921299530705785e1_f64 * t8706 * t34237;
    (t34222, t34223, t34226, t34227, t34230, t34231, t34236, t34237, t34240)
}
