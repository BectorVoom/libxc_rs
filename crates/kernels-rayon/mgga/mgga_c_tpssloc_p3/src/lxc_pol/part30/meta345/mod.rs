//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1383;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta345(t1036: f64, t4617: f64, t10422: f64, t4574: f64, t3070: f64, t1597: f64, t4509: f64, t10189: f64, t344: f64, t4343: f64, t2986: f64, t134: f64, t2978: f64, t4338: f64, t10190: f64, t4514: f64, t10213: f64, t60: f64, t135: f64, t340: f64, t4548: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13758, t13765, t13767, t13769, t13782, t13783) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1383(t1036, t4617, t10422, t4574, t3070, t1597, t4509, t10189, t344, t4343, t2986, t134, t2978);
        let (t13787, t13790, t13797, t13798, t13825) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1384(t13783, t344, t4338, t2986, t10190, t4514, t10213, t60, t135, t340, t4548, t973);
    (t13758, t13765, t13767, t13769, t13782, t13783, t13787, t13790, t13797, t13798, t13825)
}
