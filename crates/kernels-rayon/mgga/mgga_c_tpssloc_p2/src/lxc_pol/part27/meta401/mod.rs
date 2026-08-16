//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1670;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta401(t11975: f64, t11977: f64, t11981: f64, t2528: f64, t5154: f64, t172: f64, t5151: f64, t763: f64, t2535: f64, t5166: f64, t592: f64, t12461: f64, t1845: f64, t11984: f64, t1307: f64, t1388: f64, t15868: f64, t15872: f64, t15876: f64, t15878: f64, t15880: f64, t15883: f64, t3698: f64, t3914: f64, t5126: f64, t5160: f64, t5161: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15899) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1670(t11975, t11977, t11981, t2528, t5154, t172, t5151, t763, t2535, t5166, t592, t12461, t1845);
        let t15903 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1671(t11984, t1307, t1388, t15868, t15872, t15876, t15878, t15880, t15883, t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15899, t3698, t3914, t5126, t5160, t5161, t9457, t9476, t9484, t9780);
    (t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15903)
}
