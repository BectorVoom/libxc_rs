//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta528(t26248: f64, t559: f64, t1358: f64, t7715: f64, t1831: f64, t22783: f64, t5234: f64, t6951: f64, t1369: f64, t22788: f64, t5314: f64, t6952: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26249, t26251, t26255, t26257, t26258, t26260, t26262) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1869(t26248, t559, t1358, t7715, t1831, t22783, t5234, t6951, t1369, t22788, t5314, t6952);
    (t26249, t26251, t26255, t26257, t26258, t26260, t26262)
}
