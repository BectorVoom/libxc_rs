//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1406;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta363(t11588: f64, t1714: f64, t3451: f64, t3447: f64, t14818: f64, t14781: f64, t14710: f64, t1716: f64, t698: f64, t1174: f64, t3435: f64, t4889: f64, t135: f64, t4930: f64, t1420: f64, t1887: f64, t337: f64, t11570: f64, t3961: f64, t4899: f64, t11545: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15338, t15341, t15347, t15348, t15349, t15364, t15366) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1406(t11588, t1714, t3451, t3447, t14818, t14781, t14710, t1716, t698, t1174, t3435, t4889);
        let (t15374, t15376, t15382, t15390, t15394) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1407(t135, t4930, t1174, t1420, t1887, t337, t11570, t3961, t1714, t4899, t11545, t60);
    (t15338, t15341, t15347, t15348, t15349, t15364, t15366, t15374, t15376, t15382, t15390, t15394)
}
