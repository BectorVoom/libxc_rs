//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2010;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta567(t1068: f64, t4696: f64, t1597: f64, t976: f64, t1022: f64, t3966: f64, t1395: f64, t671: f64, t23862: f64, t580: f64, t23901: f64, t576: f64, t1404: f64, t7002: f64, t2029: f64, t3931: f64, t2022: f64, t3946: f64, t1372: f64, t794: f64, t6897: f64, t6907: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60941, t61066, t61774, t66940, t80593, t80597) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2010(t1068, t4696, t1597, t976, t1022, t3966, t1395, t671, t23862, t580, t23901, t576);
        let (t80599, t80601, t80605, t80645, t80647, t80650) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2011(t1404, t7002, t2029, t3931, t2022, t3946, t1372, t794, t6897, t6907, t213, t225);
    (t60941, t61066, t61774, t66940, t80593, t80597, t80599, t80601, t80605, t80645, t80647, t80650)
}
