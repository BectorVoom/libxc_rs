//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1201/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1201(t1352: f64, t3339: f64, t3351: f64, t6338: f64, t10160: f64, t23927: f64, t2317: f64, t6525: f64, t8026: f64, t1365: f64, t23983: f64, t25575: f64) -> (f64, f64, f64, f64, f64) {
    let t32067 = t3339 * t1352;
    let t32071 = t6338 * t3351;
    let t32072 = 0.11856252764865062333e-2_f64 * t32071;
    let t32073 = t23927 * t10160;
    let t32074 = 0.23712505529730124666e-2_f64 * t32073;
    let t32076 = t6525 * t8026 * t2317;
    let t32077 = 0.23712505529730124666e-2_f64 * t32076;
    let t32079 = t23983 * t1365 * t25575;
    (t32067, t32072, t32074, t32077, t32079)
}
