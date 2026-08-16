//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2176/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2176(t17157: f64, t4510: f64, t17161: f64, t13798: f64, t17152: f64, t10236: f64, t5392: f64, t10235: f64, t13851: f64, t4514: f64, t10287: f64, t10333: f64, t10339: f64, t13893: f64, t13896: f64, t13907: f64, t13909: f64, t13915: f64, t2986: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17854 = t4510 * t17157;
    let t17857 = t4510 * t17161;
    let t17860 = t13798 * t17152;
    let t17863 = t10236 * t5392;
    let t17864 = t10235 * t17863;
    let t17867 = t13851 * t4514;
    let t17873 = -t13893 - 0.12345679012345679012e-3_f64 * t13896 - 0.22222222222222222221e-2_f64 * t2986 * t17854 + 0.74074074074074074072e-3_f64 * t2986 * t17857 + 0.86419753086419753084e-3_f64 * t2986 * t17860 - 0.37037037037037037036e-3_f64 * t2986 * t17864 - 0.55555555555555555554e-3_f64 * t2986 * t17867 + 0.18518518518518518518e-3_f64 * t10287 + 0.49382716049382716048e-3_f64 * t10333 + t10339 + t13907 + 0.37037037037037037036e-3_f64 * t13909 - t13915;
    (t17854, t17857, t17860, t17863, t17864, t17867, t17873)
}
