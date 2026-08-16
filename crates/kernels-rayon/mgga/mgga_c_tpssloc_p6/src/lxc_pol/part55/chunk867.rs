//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 867/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk867(t22852: f64, t22855: f64, t2003: f64, t3862: f64, t1358: f64, t6940: f64, t1887: f64, t22715: f64, t534: f64, t1995: f64, t9223: f64, t213: f64) -> (f64, f64, f64, f64, f64) {
    let t22856 = t22852 * t22855;
    let t22858 = t2003 * t3862;
    let t22859 = 119.0_f64 / 6912.0_f64 * t22858;
    let t22860 = t6940 * t1358;
    let t22863 = t22715 * t534 * t1887;
    let t22864 = 35.0_f64 / 432.0_f64 * t22863;
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    (t22856, t22859, t22860, t22864, t22866)
}
