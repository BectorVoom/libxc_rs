//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 608/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk608(t15037: f64, t15041: f64, t15044: f64, t15047: f64, t15062: f64, t15064: f64, t15076: f64, t15079: f64, t2868: f64, t3188: f64, t3194: f64, t5928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15426 = 0.30487649791575028312e-3_f64 * t15037;
    let t15427 = 0.30487649791575028312e-3_f64 * t15041;
    let t15428 = 0.16263363996404810741e-4_f64 * t15044;
    let t15429 = 0.16263363996404810741e-4_f64 * t15047;
    let t15430 = 0.72042316457491791901e-3_f64 * t15062;
    let t15431 = 0.38430329123504567781e-4_f64 * t15064;
    let t15433 = 0.44903406381989282115e-1_f64 * t15076;
    let t15434 = 0.14967802127329760705e-1_f64 * t15079;
    let t15437 = t2868 * t3188;
    let t15438 = 0.14967802127329760705e-1_f64 * t15437;
    let t15445 = t5928 * t3194;
    (t15426, t15427, t15428, t15429, t15430, t15431, t15433, t15434, t15438, t15445)
}
