//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1446/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1446<F: Float>(t2133: F, t2563: F, t6848: F, t1554: F, t22975: F, t22978: F, t22983: F, t22985: F, t22987: F, t22991: F, t25216: F, t27219: F, t27222: F, t27229: F, t27232: F, t27234: F, t27242: F, t360: F, t5110: F, t8001: F) -> (F,) {
    let t27245 = t2133 * t6848 * t2563;
    let t27246 = 0.12713391885412927226e1 * t27245;
    let t27247 = 0.13002332610081402845e0 * t2133 * t360 * t8001 * t1554 + 0.34930954652346593433e-1 * t27219 - 0.7801399566048841707e0 * t27222 * t25216 * t5110 + t27229 - 0.22084125774650235182e1 * t27232 + 0.98171973930797904389e-1 * t27234 + 0.29272321618148349056e-1 * t22975 - 0.17465477326173296717e-1 * t22978 + 0.65854491829355115988e-1 * t22983 + 0.58903184358478742634e0 * t22985 + 0.29634521323209802195e0 * t22987 + 0.58218257753910989057e-2 * t22991 - 0.69345773920434148506e0 * t27242 + t27246;
    (t27247,)
}
