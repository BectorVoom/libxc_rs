//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 742/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk742(t5097: f64, t696: f64, t1806: f64, t5105: f64, t10449: f64, t682: f64, t11385: f64, t1814: f64, t1060: f64, t4658: f64, t5101: f64, t5100: f64, t680: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11465 = t696 * t5097;
    let t11467 = t1806 * t5105;
    let t11469 = t682 * t10449;
    let t11472 = t1814 * t11385;
    let t11476 = t5101 * t1060 * t4658;
    let t11480 = 1.0_f64 / t5100 / t680;
    (t11465, t11467, t11469, t11472, t11476, t11480)
}
