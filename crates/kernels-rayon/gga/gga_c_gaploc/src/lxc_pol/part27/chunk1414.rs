//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1414/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1414(t12270: f64, t12272: f64, t12277: f64, t1960: f64, t2208: f64, t33952: f64, t33955: f64, t33958: f64, t33963: f64, t33966: f64, t33968: f64, t33970: f64, t33973: f64, t33974: f64, t33977: f64, t33979: f64, t33980: f64, t33982: f64, t33988: f64, t3749: f64, t5549: f64, t5552: f64, t841: f64) -> f64 {
    let t38906 = 4.0_f64 * t12270 * t1960 * t841 + 4.0_f64 * t12272 * t5552 - t12277 * t2208 - t3749 * t5549 - t33952 + t33955 - t33958 - t33963 - t33966 + t33968 + t33970 - t33973 + t33974 + t33977 + t33979 - t33980 - t33982 + t33988;
    t38906
}
