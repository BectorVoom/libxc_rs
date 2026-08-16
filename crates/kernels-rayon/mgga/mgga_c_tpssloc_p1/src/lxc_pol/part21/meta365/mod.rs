//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1795;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1796;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta365(t13532: f64, t2826: f64, t136: f64, t10216: f64, t1409: f64, t2244: f64, t10304: f64, t2775: f64, t3966: f64, t607: f64, t908: f64, t2250: f64, t4342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13533, t13534, t13536, t13537) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1795(t13532, t2826, t136, t10216, t1409, t2244);
        let (t13538, t13539, t13542) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1796(t10304, t13537, t136, t2775, t3966, t607);
        let (t13543, t13544, t13546) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1797(t13542, t908, t136, t2250, t4342);
    (t13533, t13534, t13536, t13537, t13538, t13539, t13542, t13543, t13544, t13546)
}
