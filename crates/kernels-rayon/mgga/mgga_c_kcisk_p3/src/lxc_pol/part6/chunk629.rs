//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 629/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk629(t1707: f64, t8708: f64, t4881: f64, t8701: f64, t1714: f64, t1248: f64, t4893: f64, t8510: f64, t1720: f64, t8514: f64, t8518: f64, t4876: f64, t4888: f64, t7076: f64, t7122: f64, t8684: f64, t8687: f64, t8690: f64, t8702: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8709 = t1707 * t8708;
    let t8715 = t4881 * t8701;
    let t8717 = t1714 * t8708;
    let t8721 = t1248 * t4893 * t8510;
    let t8724 = t1248 * t1720 * t8514;
    let t8727 = t1248 * t1720 * t8518;
    let t8729 = -0.9494625e0_f64 * t8702 + 0.1898925e1_f64 * t8709 + t4876 + 0.19931111111111111111e0_f64 * t7076 - 0.19931111111111111111e0_f64 * t8684 + 0.59793333333333333334e0_f64 * t8687 - 0.29896666666666666667e0_f64 * t8690 + 0.15358125e0_f64 * t8715 + 0.3071625e0_f64 * t8717 + t4888 + 0.21908444444444444444e0_f64 * t7122 - 0.5477111111111111111e-1_f64 * t8721 + 0.32862666666666666666e0_f64 * t8724 - 0.16431333333333333333e0_f64 * t8727;
    (t8709, t8715, t8717, t8721, t8724, t8727, t8729)
}
