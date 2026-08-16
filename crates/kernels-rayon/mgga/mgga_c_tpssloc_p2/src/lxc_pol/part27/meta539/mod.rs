//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1965;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta539(t1874: f64, t26179: f64, t6525: f64, t7458: f64, t22751: f64, t7692: f64, t22666: f64, t7691: f64, t6888: f64, t5187: f64, t6890: f64, t6889: f64, t1834: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26181, t26183, t26184, t26186, t26187, t26189, t26190) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1965(t1874, t26179, t6525, t7458, t22751, t7692, t22666, t7691, t6888, t5187, t6890, t6889);
        let (t26191, t26193) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1966(t26190, t6888, t1834, t214);
    (t26181, t26183, t26184, t26186, t26187, t26189, t26190, t26191, t26193)
}
