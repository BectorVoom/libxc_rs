//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1950/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1950(t11613: f64, t11925: f64, t11928: f64, t1252: f64, t15808: f64, t15814: f64, t15816: f64, t15820: f64, t15823: f64, t15831: f64, t1761: f64, t3487: f64, t3593: f64, t3600: f64, t3631: f64, t4945: f64, t498: f64, t5060: f64, t5089: f64) -> f64 {
    let t15833 = -2.0_f64 * t11613 * t1761 - t11925 * t1761 - t11928 * t1761 - 2.0_f64 * t1252 * t15820 + t15808 * t498 + t15814 * t498 + 2.0_f64 * t15816 * t498 + 2.0_f64 * t15823 * t498 + t15831 * t498 - 2.0_f64 * t3487 * t5089 + 4.0_f64 * t3593 * t5060 + 2.0_f64 * t3600 * t4945 - t3631 * t4945;
    t15833
}
