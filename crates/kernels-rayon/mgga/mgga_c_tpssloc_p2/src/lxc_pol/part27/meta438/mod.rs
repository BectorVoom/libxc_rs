//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1765;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta438(t1351: f64, t22705: f64, t236: f64, t550: f64, t22852: f64, t2003: f64, t3862: f64, t1358: f64, t6940: f64, t1887: f64, t22715: f64, t534: f64, t1995: f64, t9223: f64, t213: f64, t1999: f64, t22805: f64, t22809: f64, t22820: f64, t22826: f64, t22830: f64, t22834: f64, t22837: f64, t22840: f64, t22848: f64, t22850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22855, t22856, t22859, t22860, t22861, t22863) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1765(t1351, t22705, t236, t550, t22852, t2003, t3862, t1358, t6940, t1887, t22715, t534);
        let (t22864, t22865, t22868, t22869) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1766(t22863, t1995, t9223, t213, t1999, t22805, t22809, t22820, t22826, t22830, t22834, t22837, t22840, t22848, t22850, t22856, t22859, t22861);
    (t22855, t22856, t22859, t22860, t22863, t22864, t22865, t22868, t22869)
}
