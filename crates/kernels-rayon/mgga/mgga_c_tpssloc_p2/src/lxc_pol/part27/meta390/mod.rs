//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1600;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta390(t11219: f64, t14726: f64, t136: f64, t4775: f64, t699: f64, t14736: f64, t3297: f64, t14740: f64, t14731: f64, t1113: f64, t14749: f64, t14753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14779, t14781, t14782, t14784, t14787, t14790, t14793, t14795) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1600(t11219, t14726, t136, t4775, t699, t14736, t3297, t14740, t14731, t1113, t14749, t14753);
    (t14779, t14781, t14782, t14784, t14787, t14790, t14793, t14795)
}
