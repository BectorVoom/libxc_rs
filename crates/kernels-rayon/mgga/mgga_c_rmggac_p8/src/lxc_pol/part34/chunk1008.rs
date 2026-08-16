//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1008/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1008(t77592: f64, t14498: f64, t5928: f64, t15526: f64, t2604: f64, t69745: f64, t71448: f64, t75440: f64, t77573: f64, t77575: f64, t77578: f64, t77581: f64, t77584: f64, t77585: f64, t77586: f64, t77587: f64, t77589: f64, t77590: f64, t77591: f64) -> f64 {
    let t77593 = 0.14967802127329760705e-1_f64 * t77592;
    let t77595 = 0.39914139006212695214e-1_f64 * t5928 * t14498;
    let t77596 = t2604 * t15526;
    let t77597 = 0.14967802127329760705e-1_f64 * t77596;
    let t77598 = 0.16263363996404810741e-4_f64 * t69745;
    let t77599 = t77573 + t77575 + t77578 - t77581 - t77584 - t77585 - t77586 + t71448 - t77587 + t77589 - t77590 - t77591 + t77593 + t77595 - t75440 + t77597 + t77598;
    t77599
}
