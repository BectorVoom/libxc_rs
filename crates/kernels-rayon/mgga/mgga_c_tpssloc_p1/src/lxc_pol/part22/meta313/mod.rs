//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1490;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta313(t15338: f64, t3451: f64, t3447: f64, t14818: f64, t14781: f64, t14710: f64, t1716: f64, t698: f64, t1174: f64, t3435: f64, t4889: f64, t135: f64, t4930: f64, t1420: f64, t1887: f64, t337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15339, t15341, t15347, t15348, t15349, t15363, t15364, t15366, t15372) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1490(t15338, t3451, t3447, t14818, t14781, t14710, t1716, t698, t1174, t3435, t4889, t135, t4930);
        let (t15374, t15376) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1491(t1174, t15372, t1420, t1887, t337);
    (t15339, t15341, t15347, t15348, t15349, t15363, t15364, t15366, t15372, t15374, t15376)
}
