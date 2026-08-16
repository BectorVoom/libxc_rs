//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1973;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta544(t1831: f64, t22783: f64, t5234: f64, t6951: f64, t1369: f64, t22788: f64, t5314: f64, t6952: f64, t1811: f64, t22797: f64, t22804: f64, t7709: f64, t1361: f64, t1799: f64, t22690: f64, t22792: f64, t5227: f64, t6916: f64, t1998: f64, t236: f64, t5187: f64, t6926: f64, t22784: f64, t22795: f64) -> (f64, f64, f64, f64) {
        let (t26255, t26257, t26258, t26260, t26262, t26266, t26268) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1973(t1831, t22783, t5234, t6951, t1369, t22788, t5314, t6952, t1811, t22797, t22804, t7709);
        let (t26271, t26277, t26280) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1974(t1361, t1799, t22690, t22792, t5227, t6916, t1998, t236, t5187, t6926, t22784, t22795, t26255, t26258, t26260, t26262, t26266, t26268);
    (t26257, t26271, t26277, t26280)
}
