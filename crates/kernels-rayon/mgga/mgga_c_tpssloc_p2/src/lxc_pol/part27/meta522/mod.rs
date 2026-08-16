//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta522(t1597: f64, t40: f64, t1933: f64, t23479: f64, t1015: f64, t7582: f64, t23472: f64, t343: f64, t23562: f64, t23509: f64, t3: f64, t23470: f64, t3030: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25637, t25638, t25639, t25641, t25642, t25644, t25645, t25650, t25651) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1928(t1597, t40, t1933, t23479, t1015, t7582, t23472, t343, t23562, t23509, t3, t23470, t3030);
    (t25637, t25638, t25639, t25641, t25642, t25644, t25645, t25650, t25651)
}
