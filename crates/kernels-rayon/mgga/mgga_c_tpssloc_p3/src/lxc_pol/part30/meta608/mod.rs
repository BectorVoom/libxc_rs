//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2000;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta608(t23171: f64, t23228: f64, t6572: f64, t212: f64, t6554: f64, t852: f64, t23030: f64, t23253: f64, t6555: f64, t81573: f64, t6563: f64, t81597: f64, t794: f64, t23208: f64, t1882: f64, t81686: f64, t9537: f64, t213: f64, t225: f64, t6556: f64, t81632: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82082, t82087, t82099, t82120, t82122) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2000(t23171, t23228, t6572, t212, t6554, t852, t23030, t23253, t6555, t81573, t6563, t81597);
        let (t82123, t82133, t82147, t82154, t82159, t82209) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2001(t82122, t794, t852, t23030, t23208, t1882, t81686, t9537, t213, t225, t6556, t81632);
    (t82082, t82087, t82099, t82120, t82123, t82133, t82147, t82154, t82159, t82209)
}
