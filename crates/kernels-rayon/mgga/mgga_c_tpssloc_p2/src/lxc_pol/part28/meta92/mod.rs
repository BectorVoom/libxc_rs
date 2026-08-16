//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk570;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk571;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk572;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk573;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta92(t225: f64, t562: f64, t567: f64, t214: f64, t1985: f64, t1878: f64, t1887: f64, t534: f64, t532: f64, t556: f64, t598: f64, t213: f64, t552: f64, t236: f64, t553: f64, t59: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1987, t1988, t1989, t1992) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk570(t225, t562, t567, t214, t1985, t1878, t1887, t534);
        let t1995 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk571(t532, t556);
        let (t1996, t1997, t1998) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk572(t1995, t598, t213, t225, t552);
        let t1999 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk573(t1998, t236);
        let (t2000, t2002) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk574(t1997, t1999, t553, t59);
    (t1987, t1988, t1989, t1992, t1995, t1996, t1998, t1999, t2000, t2002)
}
