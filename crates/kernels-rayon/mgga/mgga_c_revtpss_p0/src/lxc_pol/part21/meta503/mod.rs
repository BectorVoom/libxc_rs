//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2119;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2120;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta503(t11249: f64, t1668: f64, t3151: f64, t3154: f64, t3117: f64, t11795: f64, t11859: f64, t11866: f64, t11875: f64, t15859: f64, t15862: f64, t15865: f64, t15866: f64, t15888: f64, t15892: f64, t15895: f64, t15899: f64, t15906: f64, t3184: f64, t375: f64, t4834: f64, t4912: f64, t12160: f64, t4891: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t15907 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2119(t11249, t1668);
        let (t15908, t15909, t15910, t15913) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2120(t3151, t3154, t15907, t3117, t11795, t11859, t11866, t11875, t15859, t15862, t15865, t15866, t15888, t15892, t15895, t15899, t15906, t3184, t375, t4834, t4912);
        let t15917 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2121(t12160, t4891);
    (t15907, t15908, t15909, t15910, t15913, t15917)
}
