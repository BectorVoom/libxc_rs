//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1185/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1185(t10227: f64, t23927: f64, t10276: f64, t4141: f64, t2321: f64, t27071: f64, t9074: f64, t10555: f64, t169: f64, t31548: f64, t6490: f64, t1365: f64, t25735: f64, t6525: f64) -> (f64, f64, f64, f64, f64) {
    let t32009 = t23927 * t10227;
    let t32010 = 0.23712505529730124666e-2_f64 * t32009;
    let t32012 = 0.9485002211892049866e-2_f64 * t4141 * t10276;
    let t32020 = t9074 * t27071 * t2321;
    let t32021 = 0.11856252764865062333e-2_f64 * t32020;
    let t32025 = 0.68292015925622759036e0_f64 * t31548 * t10555 * t169 * t6490;
    let t32027 = t6525 * t1365 * t25735;
    (t32010, t32012, t32021, t32025, t32027)
}
