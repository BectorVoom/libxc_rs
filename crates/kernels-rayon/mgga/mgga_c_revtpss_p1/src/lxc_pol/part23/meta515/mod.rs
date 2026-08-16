//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2020;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta515(t1222: f64, t1266: f64, t12784: f64, t12855: f64, t17437: f64, t21121: f64, t21126: f64, t21129: f64, t21134: f64, t21137: f64, t21140: f64, t21143: f64, t5304: f64, t5309: f64, t5313: f64, t5373: f64, t5391: f64, t6640: f64, t1264: f64, t20272: f64, t247: f64, t5405: f64, t6429: f64, t3626: f64, t6425: f64, t1794: f64, t5245: f64, t1250: f64, t3720: f64, t140: f64, t6652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t21146 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2020(t1222, t1266, t12784, t12855, t17437, t21121, t21126, t21129, t21134, t21137, t21140, t21143, t5304, t5309, t5313, t5373, t5391, t6640);
        let (t21153, t21156, t21157, t21160, t21161, t21164, t21165, t21166, t21169) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2021(t1264, t20272, t247, t5405, t6429, t3626, t6425, t1794, t5245, t1250, t3720, t140, t6652);
    (t21146, t21153, t21156, t21157, t21160, t21161, t21164, t21165, t21166, t21169)
}
