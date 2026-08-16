//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1820;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta414(t3108: f64, t4640: f64, t1611: f64, t3047: f64, t3103: f64, t4641: f64, t1040: f64, t4616: f64, t1044: f64, t13611: f64, t248: f64, t1023: f64, t13975: f64, t4582: f64, t3121: f64, t4593: f64, t3041: f64, t1031: f64, t1612: f64, t3082: f64, t1025: f64, t1041: f64, t1046: f64, t10873: f64, t10883: f64, t10952: f64, t10965: f64, t1622: f64, t3039: f64, t3048: f64, t3117: f64, t378: f64, t4585: f64, t4590: f64, t4600: f64, t4636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14077, t14080, t14084, t14085, t14093, t14098) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1820(t3108, t4640, t1611, t3047, t3103, t4641, t1040, t4616, t1044, t13611, t248, t1023, t13975);
        let (t14099, t14102, t14103, t14106, t14107, t14114, t14120) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1821(t14098, t4582, t3121, t4593, t3041, t1031, t4616, t1612, t3082, t1025, t1041, t1046, t10873, t10883, t10952, t10965, t14077, t14080, t14084, t14085, t14093, t1622, t3039, t3048, t3117, t378, t4585, t4590, t4600, t4636);
    (t14077, t14080, t14085, t14093, t14098, t14099, t14102, t14103, t14106, t14107, t14114, t14120)
}
