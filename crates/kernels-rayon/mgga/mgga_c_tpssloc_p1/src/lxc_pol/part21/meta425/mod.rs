//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1949;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta425(t1137: f64, t15117: f64, t1147: f64, t4832: f64, t1687: f64, t3400: f64, t1156: f64, t14829: f64, t3375: f64, t1129: f64, t11356: f64, t1148: f64, t1157: f64, t14840: f64, t14847: f64, t14849: f64, t14852: f64, t1695: f64, t3371: f64, t3378: f64, t3396: f64, t3404: f64, t4835: f64, t4858: f64, t1128: f64, t4794: f64, t1675: f64, t3356: f64, t1136: f64, t4820: f64, t1683: f64, t3351: f64, t3333: f64, t4823: f64, t1138: f64, t11410: f64, t11420: f64, t14864: f64, t14866: f64, t14916: f64, t14934: f64, t14939: f64, t3327: f64, t3332: f64, t3352: f64, t3360: f64, t4797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15118, t15121, t15126, t15133, t15136, t15139) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1949(t1137, t15117, t1147, t4832, t1687, t3400, t1156, t14829, t3375, t1129, t11356, t1148, t1157, t14840, t14847, t14849, t14852, t1695, t3371, t3378, t3396, t3404, t4835, t4858);
        let (t15141, t15146, t15153, t15156, t15159, t15162) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1950(t1128, t4794, t1675, t3356, t1136, t4820, t1683, t3351, t3333, t4823, t1138, t11410, t11420, t14864, t14866, t14916, t14934, t14939, t3327, t3332, t3352, t3360, t4797);
    (t15118, t15121, t15126, t15133, t15136, t15139, t15141, t15146, t15153, t15156, t15159, t15162)
}
