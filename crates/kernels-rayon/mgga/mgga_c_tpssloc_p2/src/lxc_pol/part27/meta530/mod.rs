//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1943;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta530(t25: f64, t1409: f64, t1965: f64, t25398: f64, t25883: f64, t3966: f64, t40: f64, t607: f64, t6835: f64, t7643: f64, t28: f64, t870: f64, t4255: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t16596: f64, t23788: f64, t1081: f64, t1484: f64, t4119: f64, t25365: f64, t10143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25890, t25891, t25892) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1943(t25, t1409, t1965, t25398, t25883, t3966, t40, t607, t6835, t7643, t28, t870, t4255, dens_threshold, rho0, zeta_threshold);
        let (t25898, t25901, t25905, t25921, t25927) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1944(t16596, t23788, t1081, t1484, t28, t4119, t25365, t10143);
    (t25890, t25891, t25892, t25898, t25901, t25905, t25921, t25927)
}
