//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2007;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta499(t1484: f64, t868: f64, t13115: f64, t157: f64, t1504: f64, t68: f64, t1499: f64, t4290: f64, t4166: f64, t4177: f64, t2632: f64, t4233: f64, t4280: f64, t3131: f64, t4649: f64, t1539: f64, t6733: f64, t3508: f64, t5011: f64, t1441: f64, t671: f64, t1388: f64, t1799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16596, t16693, t16729, t16830, t16836, t16935) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2007(t1484, t868, t13115, t157, t1504, t68, t1499, t4290, t4166, t4177, t2632, t4233);
        let (t17034, t17732, t17748, t18946, t19456, t19577) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2008(t1499, t4280, t3131, t4649, t1539, t6733, t3508, t5011, t1441, t671, t1388, t1799);
    (t16596, t16693, t16729, t16830, t16836, t16935, t17034, t17732, t17748, t18946, t19456, t19577)
}
