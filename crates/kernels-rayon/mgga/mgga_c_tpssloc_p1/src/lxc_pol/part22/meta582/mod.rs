//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta582(t11888: f64, t45113: f64, t11914: f64, t11784: f64, t820: f64, t11779: f64, t11677: f64, t11907: f64, t11904: f64, t11153: f64, t1176: f64, t11881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45114, t45119, t45124, t45128, t45134, t45162, t45192, t45197) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2092(t11888, t45113, t11914, t11784, t820, t11779, t11677, t11907, t11904, t11153, t1176, t11881);
    (t45114, t45119, t45124, t45128, t45134, t45162, t45192, t45197)
}
