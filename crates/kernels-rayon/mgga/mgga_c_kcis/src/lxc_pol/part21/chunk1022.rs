//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1022/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1022(t2635: f64, t5324: f64, t3515: f64, t11034: f64, t11042: f64, t11063: f64, t11066: f64, t11070: f64, t11083: f64, t11093: f64, t11098: f64, t11100: f64, t15502: f64, t15513: f64, t15518: f64, t15521: f64, t3514: f64) -> f64 {
    let t15524 = t5324 * t2635;
    let t15525 = t3515 * t15524;
    let t15528 = t3514 * t15502 / 144.0_f64 - t11034 / 324.0_f64 - t11042 / 864.0_f64 - t11063 / 1296.0_f64 + t11066 / 1728.0_f64 + t11070 / 1296.0_f64 - t11083 / 864.0_f64 + t11093 + 11.0_f64 / 648.0_f64 * t11098 + t11100 / 162.0_f64 - t3514 * t15513 / 72.0_f64 - t15518 - t3514 * t15521 / 288.0_f64 - t3514 * t15525 / 576.0_f64;
    t15528
}
