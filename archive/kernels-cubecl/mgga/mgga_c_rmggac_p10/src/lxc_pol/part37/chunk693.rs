//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 693/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk693<F: Float>(t328: F, t69144: F, t14301: F, t25640: F, t14286: F, t321: F, t262: F, t7788: F, t333: F, t7782: F, t68685: F, t7835: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69145 = t69144 * t328;
    let t69146 = F::cast_from(0.10909864661698136691e0_f64) * t69145;
    let t69151 = t25640 * t14301;
    let t69156 = t14286 * t321;
    let t69157 = t262 * t69156;
    let t69158 = t7788 * t69157;
    let t69160 = t14286 * t333;
    let t69161 = t262 * t69160;
    let t69162 = t7782 * t69161;
    let t69164 = t7835 * t68685;
    (t69146, t69151, t69156, t69157, t69158, t69160, t69161, t69162, t69164)
}
