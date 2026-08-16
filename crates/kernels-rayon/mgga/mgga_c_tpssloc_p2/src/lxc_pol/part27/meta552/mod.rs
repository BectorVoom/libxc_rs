//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta552(t1998: f64, t5318: f64, t214: f64, t1985: f64, t7740: f64, t794: f64, t6897: f64, t1825: f64, t22873: f64, t552: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26432, t26433, t26434, t26436, t26437, t26442, t26446) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1990(t1998, t5318, t214, t1985, t7740, t794, t6897, t1825, t22873, t552, t6604);
    (t26432, t26433, t26434, t26436, t26437, t26442, t26446)
}
