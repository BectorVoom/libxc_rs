//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta358(t14722: f64, t14704: f64, t1667: f64, t2403: f64, t14720: f64, t4775: f64, t699: f64, t4772: f64, t1657: f64, t3263: f64, t1098: f64, t4737: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14723, t14724, t14766, t14768, t14781, t14782, t14818, t14838, t14845) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1400(t14722, t14704, t1667, t2403, t14720, t4775, t699, t4772, t1657, t3263, t1098, t4737);
    (t14723, t14724, t14766, t14768, t14781, t14782, t14818, t14838, t14845)
}
