//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3149/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3149<F: Float>(t16862: F, t3399: F, t12322: F, t5087: F, t12328: F, t1723: F, t43821: F, t43946: F, t56176: F, t56183: F, t43830: F, t43832: F, t43881: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56181: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F) -> (F, F, F, F, F) {
    let t58055 = t16862 * t3399;
    let t58057 = t5087 * t12322;
    let t58060 = t43821 * t1723 * t12328;
    let t58063 = t43946 * t1723 * t12328;
    let t58073 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t56176;
    let t58075 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t56183;
    let t58084 = t43881 - F::cast_from(8.0_f64) * t56151 + F::cast_from(2.0_f64) * t56155 + F::cast_from(6.0_f64) * t56159 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t56163 + F::cast_from(8.0_f64) * t56167 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t43830 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43832 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t56174 - t58073 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t56181 + t58075 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t56185 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t56187 - F::cast_from(2.0_f64) * t56189 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t56194 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t56198 - F::cast_from(4.0_f64) * t56203 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t56207 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t56209;
    (t58055, t58057, t58060, t58063, t58084)
}
