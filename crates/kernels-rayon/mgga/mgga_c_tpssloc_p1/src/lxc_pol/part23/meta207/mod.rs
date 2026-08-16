//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk849;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta207(t12214: f64, t205: f64, t116: f64, t547: f64, t535: f64, t9534: f64, t9538: f64, t1337: f64, t562: f64, t3792: f64, t550: f64, t1339: f64, t836: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12215, t12225, t12236, t12247, t12248, t12249, t12250, t12282) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk849(t12214, t205, t116, t547, t535, t9534, t9538, t1337, t562, t3792, t550, t1339, t836);
        let t12283 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk850(t12282, t1336);
    (t12215, t12225, t12236, t12247, t12248, t12249, t12250, t12282, t12283)
}
