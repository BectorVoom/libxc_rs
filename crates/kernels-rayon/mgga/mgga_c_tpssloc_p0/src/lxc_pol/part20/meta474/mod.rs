//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1947;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta474(t15030: f64, t15785: f64, t1241: f64, t1251: f64, t5088: f64, t3598: f64, t1760: f64, t3599: f64, t11606: f64, t225: f64, t4941: f64, t1751: f64, t3481: f64, t3630: f64, t1238: f64, t1252: f64, t14972: f64, t14980: f64, t3487: f64, t3593: f64, t3600: f64, t3631: f64, t498: f64, t5055: f64, t5060: f64, t5089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15786, t15787, t15790, t15794, t15797, t15800) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1947(t15030, t15785, t1241, t1251, t5088, t3598, t1760, t3599, t11606, t225, t4941, t1751, t3481);
        let (t15803, t15806) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1948(t1760, t3630, t3598, t1238, t1252, t14972, t14980, t15787, t15790, t15794, t15797, t15800, t3487, t3593, t3600, t3631, t498, t5055, t5060, t5089);
    (t15786, t15787, t15790, t15794, t15797, t15800, t15803, t15806)
}
