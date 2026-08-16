//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1403;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta326(t121: f64, t3584: f64, t248: f64, t3243: f64, t1227: f64, t1229: f64, t676: f64, t1090: f64, t3536: f64, t3572: f64, t3252: f64, t3521: f64, t3248: f64, t1009: f64, t3481: f64, t1011: f64, t1212: f64, t486: f64, t1216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11787, t11789, t11792, t11794, t11797) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1403(t121, t3584, t248, t3243, t1227, t1229, t676, t1090, t3536, t3572, t3252, t3521);
        let (t11798, t11802, t11812, t11814, t11818, t11820) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1404(t11797, t1227, t248, t3248, t3521, t1009, t3481, t1011, t1212, t486, t676, t1216);
    (t11787, t11789, t11792, t11794, t11798, t11802, t11812, t11814, t11818, t11820)
}
