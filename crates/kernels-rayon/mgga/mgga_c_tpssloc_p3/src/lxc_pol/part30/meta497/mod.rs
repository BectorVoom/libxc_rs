//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1812;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta497(t1599: f64, t23588: f64, t23384: f64, t7554: f64, t1065: f64, t7624: f64, t3174: f64, t7614: f64, t986: f64, t6805: f64, t7607: f64, t1949: f64, t4542: f64, t225: f64, t7577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25447, t25450, t25453, t25456, t25459, t25465, t25467) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1812(t1599, t23588, t23384, t7554, t1065, t7624, t3174, t7614, t986, t6805, t7607, t1949, t4542);
        let t25470 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1813(t225, t7577);
    (t25447, t25450, t25453, t25456, t25459, t25465, t25467, t25470)
}
