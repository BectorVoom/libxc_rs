//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 910/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk910(t23139: f64, t8339: f64, t23171: f64, t23228: f64, t8335: f64, t1902: f64, t213: f64, t225: f64, t23030: f64, t30638: f64, t212: f64, t6554: f64) -> (f64, f64, f64, f64, f64) {
    let t112855 = t23139 * t8339;
    let t112863 = 0.16449340668482264365e-1_f64 * t23171 * t23228 * t8335;
    let t112899 = t213 * t1902 * t225;
    let t112936 = 0.52089578783527170489e-1_f64 * t23030 * t30638;
    let t112942 = 0.16449340668482264365e-1_f64 * t23171 * t212 * t1902 * t6554;
    (t112855, t112863, t112899, t112936, t112942)
}
