//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 999/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk999(t157: f64, t9929: f64, t4196: f64, t9726: f64, t10143: f64, t1530: f64, t2430: f64, t4205: f64, t1409: f64, t750: f64, t607: f64, t4194: f64) -> (f64, f64, f64, f64, f64) {
    let t12908 = t9929 * t157;
    let t12910 = 24.0_f64 * t12908 * t4196;
    let t12914 = 2.0_f64 * t9726;
    let t12915 = t1530 * t10143;
    let t12922 = 8.0_f64 * t4205 * t2430;
    let t12923 = t750 * t1409;
    let t12924 = t12923 * t607;
    let t12926 = 24.0_f64 * t4194 * t12924;
    (t12910, t12914, t12915, t12922, t12926)
}
