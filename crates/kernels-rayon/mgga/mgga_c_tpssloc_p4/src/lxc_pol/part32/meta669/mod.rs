//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2101;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta669(t27498: f64, t85853: f64, t27533: f64, t86094: f64, t24826: f64, t27521: f64, t24574: f64, t27462: f64, t3030: f64, t460: f64, t27488: f64, t27491: f64, t27495: f64, t27497: f64, t1170: f64, t2121: f64, t27732: f64, t15590: f64, t7338: f64, t27614: f64, t3572: f64, t27617: f64, t3523: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95136, t95163, t95165, t95192, t95195, t95197) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2101(t27498, t85853, t27533, t86094, t24826, t27521, t24574, t27462, t3030, t460, t27488, t27491);
        let (t95201, t95213, t95238, t95242, t95244) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2102(t27495, t27497, t95195, t1170, t2121, t27732, t15590, t7338, t27614, t3572, t27617, t3523);
    (t95136, t95163, t95165, t95192, t95197, t95201, t95213, t95238, t95242, t95244)
}
