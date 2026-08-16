//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta432(t15338: f64, t3451: f64, t3447: f64, t14818: f64, t14781: f64, t14710: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11487: f64, t14713: f64, t14766: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t15339, t15341, t15347, t15348, t15349, t15357) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1966(t15338, t3451, t3447, t14818, t14781, t14710, t11211, t11213, t11215, t11217, t11487, t14713, t14766, t14779, t14784, t14787, t14790, t14793, t14796, t14799);
    (t15339, t15341, t15347, t15348, t15349, t15357)
}
