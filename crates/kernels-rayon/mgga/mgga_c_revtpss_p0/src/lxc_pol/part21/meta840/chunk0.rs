//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3149/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3149(t16862: f64, t3399: f64, t12322: f64, t5087: f64, t12328: f64, t1723: f64, t43821: f64, t43946: f64, t56176: f64, t56183: f64, t43830: f64, t43832: f64, t43881: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64) -> (f64, f64, f64, f64, f64) {
    let t58055 = t16862 * t3399;
    let t58057 = t5087 * t12322;
    let t58060 = t43821 * t1723 * t12328;
    let t58063 = t43946 * t1723 * t12328;
    let t58073 = 8.0_f64 / 27.0_f64 * t56176;
    let t58075 = 8.0_f64 / 9.0_f64 * t56183;
    let t58084 = t43881 - 8.0_f64 * t56151 + 2.0_f64 * t56155 + 6.0_f64 * t56159 + 2.0_f64 / 3.0_f64 * t56163 + 8.0_f64 * t56167 - 2.0_f64 / 3.0_f64 * t43830 + 2.0_f64 / 9.0_f64 * t43832 - 80.0_f64 / 81.0_f64 * t56174 - t58073 + 40.0_f64 / 9.0_f64 * t56181 + t58075 - 4.0_f64 / 3.0_f64 * t56185 - 2.0_f64 / 3.0_f64 * t56187 - 2.0_f64 * t56189 - 2.0_f64 / 3.0_f64 * t56194 - 2.0_f64 / 3.0_f64 * t56198 - 4.0_f64 * t56203 - 2.0_f64 / 9.0_f64 * t56207 + 4.0_f64 / 9.0_f64 * t56209;
    (t58055, t58057, t58060, t58063, t58084)
}
