//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2238;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta548(t136: f64, t18517: f64, t1113: f64, t18241: f64, t11211: f64, t11487: f64, t14766: f64, t15347: f64, t15348: f64, t15349: f64, t18494: f64, t18497: f64, t18500: f64, t18503: f64, t18505: f64, t18508: f64, t18510: f64, t18512: f64, t18515: f64, t457: f64, t460: f64, t974: f64, t135: f64, t6146: f64, t1174: f64, t6140: f64, t11558: f64, t15341: f64, t15364: f64, t15366: f64, t15374: f64, t15376: f64, t18475: f64, t18484: f64, t18489: f64, t3447: f64, t4905: f64, t4909: f64, t4920: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t18518, t18520, t18521, t18523) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2238(t136, t18517, t1113, t18241, t11211, t11487, t14766, t15347, t15348, t15349, t18494, t18497, t18500, t18503, t18505, t18508, t18510, t18512, t18515);
        let (t18525, t18535) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2239(t18523, t457, t460, t974, t135, t6146, t1174, t6140, t11558, t15341, t15364, t15366, t15374, t15376, t18475, t18484, t18489, t3447, t4905, t4909, t4920);
    (t18518, t18520, t18521, t18523, t18525, t18535)
}
