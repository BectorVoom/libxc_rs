//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1346;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta322(t11707: f64, t3032: f64, t3505: f64, t10469: f64, t466: f64, t10471: f64, t1208: f64, t478: f64, t10477: f64, t483: f64, t3508: f64, t475: f64, t3503: f64, t3514: f64, t1210: f64, t3247: f64, t415: f64, t121: f64, t3584: f64, t1229: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11708, t11709, t11712, t11713, t11715, t11717, t11719, t11721) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1346(t11707, t3032, t3505, t10469, t466, t10471, t1208, t478, t10477, t483, t3508, t475);
        let (t11728, t11734, t11738, t11778, t11784, t11789) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1347(t11717, t3503, t11713, t11708, t3514, t1210, t3247, t415, t121, t3584, t1229, t676);
    (t11709, t11712, t11715, t11719, t11721, t11728, t11734, t11738, t11778, t11784, t11789)
}
