//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1748;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta348(t13003: f64, t13028: f64, t252: f64, t1492: f64, t2710: f64, t1519: f64, t2591: f64, t225: f64, t4266: f64, t10049: f64, t1528: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t2743: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t866: f64, t9590: f64, t9593: f64, t1527: f64, t2719: f64, t10110: f64, t4143: f64, t2742: f64, t2718: f64, t4265: f64, t798: f64, t4145: f64, t4142: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13029, t13030, t13034, t13036, t13042, t13048) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1748(t13003, t13028, t252, t1492, t2710, t1519, t2591, t225, t4266, t10049, t1528, t259, t2597, t2713, t2720, t2743, t4147, t4268, t4273, t4301, t866, t9590, t9593);
        let (t13050, t13053, t13059, t13062, t13065, t13068) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1749(t1527, t2719, t10110, t225, t4143, t2742, t2718, t4265, t798, t4145, t4142, t852);
    (t13029, t13030, t13034, t13036, t13042, t13048, t13050, t13053, t13059, t13062, t13065, t13068)
}
