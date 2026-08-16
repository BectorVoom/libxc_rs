//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1457;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1458;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1459;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1460;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta350(t1484: f64, t212: f64, t9523: f64, t2586: f64, t213: f64, t4119: f64, t221: f64, t776: f64, t2553: f64, t4128: f64, t2570: f64, t67: f64, t792: f64, t686: f64, t4127: f64, t9526: f64, t9540: f64, t9542: f64, t9544: f64, t9547: f64, t9552: f64, t9556: f64, t131: f64, t9558: f64, t205: f64, t2379: f64, t1489: f64, t9541: f64, t4126: f64, t782: f64, t4130: f64, t12971: f64, t210: f64, t214: f64, t2563: f64, t4138: f64, t4134: f64, t9546: f64, t118: f64, t794: f64, t2576: f64, t787: f64, t9572: f64, t9574: f64, t9579: f64, t9583: f64, t252: f64, t1492: f64, t2710: f64, t1519: f64, t2591: f64, t225: f64, t4266: f64, t10049: f64, t1528: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t2743: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t866: f64, t9590: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12984, t12986, t12990, t12994, t12997) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1457(t1484, t212, t9523, t2586, t213, t4119, t221, t776, t2553, t4128, t2570, t67);
        let t13003 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1458(t12997, t792, t12984, t686, t776, t12986, t12990, t12994, t4127, t9526, t9540, t9542, t9544, t9547, t9552, t9556);
        let (t13005, t13007, t13010, t13014, t13017) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1459(t131, t9558, t205, t221, t2379, t4128, t1489, t9541, t4126, t782, t4130, t12971, t210, t214);
        let t13028 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1460(t2563, t4138, t4134, t9546, t118, t4119, t794, t2576, t13005, t13007, t13010, t13014, t13017, t787, t9572, t9574, t9579, t9583);
        let (t13029, t13030, t13034, t13036, t13042, t13048) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1461(t13003, t13028, t252, t1492, t2710, t1519, t2591, t225, t4266, t10049, t1528, t259, t2597, t2713, t2720, t2743, t4147, t4268, t4273, t4301, t866, t9590, t9593);
    (t13029, t13030, t13034, t13036, t13042, t13048)
}
