//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1678;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1679;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1680;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta404(t25: f64, t1788: f64, t2225: f64, t2221: f64, t2223: f64, t12130: f64, t11987: f64, t1408: f64, t2: f64, t3704: f64, t1298: f64, t15941: f64, t16: f64, t2249: f64, t3665: f64, t5170: f64, t5173: f64, t584: f64, zeta_threshold: f64, t28: f64, t12000: f64, t1649: f64, t3711: f64, t1302: f64, t15956: f64, t3231: f64, t3673: f64, t5178: f64, t5181: f64, t225: f64, t5213: f64, t1807: f64, t3879: f64, t5211: f64, t1332: f64, t5343: f64, t1372: f64, t1824: f64, t5250: f64, t5286: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15983, t15985, t15987, t15988, t16002) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1678(t25, t1788, t2225, t2221, t2223, t12130, t11987, t1408, t2, t3704, t1298, t15941, t16, t2249, t3665, t5170, t5173, t584, zeta_threshold);
        let t16016 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1679(t28, t12000, t1649, t2, t3711, t1302, t15956, t16, t3231, t3673, t5178, t5181, t584, zeta_threshold);
        let t16018 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1680(t16002, t16016);
        let (t16022, t16028, t16030, t16033, t16036, t16037, t16040) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1681(t225, t5213, t1807, t3879, t5211, t1332, t5343, t1372, t1824, t5250, t5286, t562);
    (t15983, t15985, t15987, t15988, t16018, t16022, t16028, t16030, t16033, t16036, t16037, t16040)
}
