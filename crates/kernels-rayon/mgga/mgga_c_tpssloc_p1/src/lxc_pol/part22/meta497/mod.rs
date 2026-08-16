//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1927;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta497(t1539: f64, t5878: f64, t3071: f64, t10930: f64, t20234: f64, t974: f64, t20217: f64, t998: f64, t10942: f64, t21510: f64, t4583: f64, t4582: f64, t1041: f64, t10413: f64, t14117: f64, t14160: f64, t14203: f64, t1618: f64, t17885: f64, t17907: f64, t18005: f64, t18008: f64, t18030: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21531, t21532, t21537, t21538, t21541, t21542, t21545, t21546, t21550, t21551) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1927(t1539, t5878, t3071, t10930, t20234, t974, t20217, t998, t10942, t21510, t4583, t4582);
        let t21560 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1928(t1041, t10413, t14117, t14160, t14203, t1618, t17885, t17907, t18005, t18008, t18030, t21532, t21538, t21542, t21546, t21551, t973);
    (t21531, t21532, t21537, t21538, t21541, t21542, t21545, t21546, t21550, t21551, t21560)
}
