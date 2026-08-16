//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta29 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk206;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk207;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk208;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk209;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk210;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk211;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk212;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta29(t544: f64, t554: f64, t532: f64, t242: f64, t248: f64, t219: f64, t541: f64, t539: f64, t553: f64, t254: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t555, t556) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk206(t544, t554, t532);
        let t557 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk207(t556);
        let t559 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk208(t242, t248, t557);
        let t562 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk209(t219, t541, t555, t559);
        let (t563, t564) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk210(t539, t562, t553);
        let (t566, t567) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk211(t544, t564);
        let t568 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk212(t254, t567);
        let (t570, t571) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk213(t563, t568);
    (t555, t556, t557, t559, t562, t563, t564, t566, t567, t568, t570, t571)
}
