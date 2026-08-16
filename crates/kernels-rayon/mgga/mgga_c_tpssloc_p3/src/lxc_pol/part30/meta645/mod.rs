//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2057;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta645(t1611: f64, t23528: f64, t23436: f64, t4640: f64, t14507: f64, t23536: f64, t23540: f64, t23433: f64, t4630: f64, t10189: f64, t1920: f64, t4343: f64, t13783: f64, t4338: f64, t14192: f64, t6717: f64, t13965: f64, t6755: f64, t25577: f64, t3103: f64, t1933: f64, t23479: f64, t88405: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88584, t88591, t88594, t88600, t88604, t88622) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2057(t1611, t23528, t23436, t4640, t14507, t23536, t23540, t23433, t4630, t10189, t1920, t4343);
        let (t88625, t88636, t88645, t88648, t88689) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2058(t13783, t1920, t4338, t14192, t6717, t13965, t6755, t25577, t3103, t1933, t23479, t88405);
    (t88584, t88591, t88594, t88600, t88604, t88622, t88625, t88636, t88645, t88648, t88689)
}
