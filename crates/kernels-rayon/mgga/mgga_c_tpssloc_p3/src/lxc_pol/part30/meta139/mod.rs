//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk767;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta139(t2375: f64, t3684: f64, t1294: f64, t2371: f64, t2528: f64, t1284: f64, t172: f64, t763: f64, t2535: f64, t570: f64, t515: f64, t518: f64, t215: f64, t2559: f64, t535: f64, t1314: f64, t782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3686, t3688, t3690, t3691, t3692, t3695, t3700, t3701) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk767(t2375, t3684, t1294, t2371, t2528, t1284, t172, t763, t2535, t570);
        let (t3704, t3711, t3725, t3726) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk768(t515, t518, t215, t2559, t535, t1314, t782);
    (t3686, t3688, t3690, t3691, t3692, t3695, t3700, t3701, t3704, t3711, t3725, t3726)
}
