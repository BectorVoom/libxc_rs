//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2061;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta597(t23272: f64, t81651: f64, t82074: f64, t23204: f64, t23218: f64, t6562: f64, t23171: f64, t23228: f64, t6572: f64, t212: f64, t6554: f64, t852: f64, t23030: f64, t23253: f64, t23241: f64, t81640: f64, t23273: f64, t81591: f64, t6555: f64, t81573: f64, t6563: f64, t81597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82076, t82079, t82082, t82087) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2061(t23272, t81651, t82074, t23204, t23218, t6562, t23171, t23228, t6572, t212, t6554, t852);
        let (t82099, t82108, t82115, t82120, t82122) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2062(t23030, t23253, t23204, t23241, t81640, t23273, t81591, t23228, t6555, t81573, t6563, t81597);
    (t82076, t82079, t82082, t82087, t82099, t82108, t82115, t82120, t82122)
}
