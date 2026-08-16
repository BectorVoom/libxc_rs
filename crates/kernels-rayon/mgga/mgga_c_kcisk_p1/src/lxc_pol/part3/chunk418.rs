//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 418/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk418(t3088: f64, t944: f64, t151: f64, t3107: f64, t852: f64, t180: f64, t182: f64, t183: f64, t2925: f64, t3144: f64, t3148: f64, t3155: f64, t3156: f64, t60: f64, t983: f64, t990: f64, t991: f64, t995: f64) -> (f64, f64, f64) {
    let t3162 = t944 * t3088;
    let t3166 = t151 * t3107;
    let t3170 = t852 * t852;
    let t3174 = -0.43802864444444444443e-3_f64 * t180 * t3144 * t183 - 0.2e-22_f64 * t990 * t3148 * t183 - 0.26281718666666666666e-2_f64 * t180 * t983 * t995 + 0.19711288999999999999e-2_f64 * t3155 * t3156 + 0.19711288999999999999e-2_f64 * t990 * t991 * t995 + 0.39422577999999999998e-2_f64 * t180 * t182 * t3162 - 0.19711288999999999999e-2_f64 * t180 * t182 * t3166 - 4.0_f64 * t3170 - 4.0_f64 * t60 * t2925;
    (t3162, t3166, t3174)
}
