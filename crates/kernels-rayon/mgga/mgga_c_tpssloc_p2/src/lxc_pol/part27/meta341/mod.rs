//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta341(t3866: f64, t3872: f64, t3876: f64, t1339: f64, t2690: f64, t1336: f64, t1354: f64, t1307: f64, t3792: f64, t3788: f64, t835: f64, t3795: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12356, t12358, t12365, t12366, t12369, t12386) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1422(t3866, t3872, t3876, t1339, t2690, t1336, t1354, t1307, t3792, t3788, t835, t3795);
    (t12356, t12358, t12365, t12366, t12369, t12386)
}
