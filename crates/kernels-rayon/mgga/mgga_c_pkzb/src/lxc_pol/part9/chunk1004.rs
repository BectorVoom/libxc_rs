//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1004/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1004(t7979: f64, t7982: f64, t6177: f64, t6218: f64, t7970: f64, t7973: f64, t7975: f64, t7986: f64, t7990: f64, t7994: f64, t7997: f64, t8000: f64) -> f64 {
    let t8090 = 0.33114e0_f64 * t7979;
    let t8091 = 0.33114e0_f64 * t7982;
    let t8097 = -0.1294625e1_f64 * t7970 + 0.16504875e0_f64 * t7973 + 0.82524375e-1_f64 * t7975 - t6218 + 0.5519e0_f64 * t6177 - t8090 - t8091 + 0.248355e0_f64 * t7986 + 0.49671e0_f64 * t7990 + 0.248355e0_f64 * t7994 + 0.19419375e1_f64 * t7997 - 0.412621875e-1_f64 * t8000;
    t8097
}
