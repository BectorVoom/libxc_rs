//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1358;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta331(t1307: f64, t212: f64, t12225: f64, t2586: f64, t535: f64, t9534: f64, t9538: f64, t1337: f64, t3792: f64, t550: f64, t1339: f64, t836: f64, t1336: f64, t3777: f64, t3789: f64, t236: f64, t3798: f64, t12189: f64, t1329: f64, t1333: f64, t3862: f64, t10022: f64, t248: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12228, t12236, t12248, t12250, t12282) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1358(t1307, t212, t12225, t2586, t535, t9534, t9538, t1337, t3792, t550, t1339, t836);
        let (t12283, t12286, t12289, t12300, t12308, t12325, t12328) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1359(t12282, t1336, t3777, t3789, t12248, t236, t3798, t12189, t1329, t1333, t3862, t10022, t248, t557);
    (t12228, t12236, t12248, t12250, t12283, t12286, t12289, t12300, t12308, t12325, t12328)
}
