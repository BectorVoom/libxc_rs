//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1583;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1584;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta440(t23228: f64, t6554: f64, t23171: f64, t23168: f64, t6556: f64, t6547: f64, t6573: f64, t214: f64, t852: f64, t6568: f64, t23030: f64, t6563: f64, t6567: f64, t794: f64, t6562: f64, t1883: f64, t23012: f64, t213: f64, t225: f64, t252: f64, t2752: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23229, t23230, t23233, t23236, t23237) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1583(t23228, t6554, t23171, t23168, t6556, t6547, t6573, t214, t852);
        let (t23250, t23251, t23253, t23254, t23261, t23270) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1584(t6547, t6568, t23030, t6563, t6567, t794, t6562, t1883, t23012, t213, t225, t252);
        let t23788 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1585(t2752, t28);
    (t23229, t23230, t23233, t23236, t23237, t23250, t23251, t23253, t23254, t23261, t23270, t23788)
}
