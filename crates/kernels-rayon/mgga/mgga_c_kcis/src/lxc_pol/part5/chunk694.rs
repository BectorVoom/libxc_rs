//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 694/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk694(t359: f64, t41: f64, t4818: f64, t5046: f64, t1184: f64, t1817: f64, t1175: f64, t1800: f64, t1170: f64, t1176: f64, t1797: f64, t1166: f64, t1805: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5047 = t41 * t359;
    let t5048 = t5047 * t4818;
    let t5049 = t5046 * t5048;
    let t5051 = t1184 * t1817;
    let t5053 = t1175 * t1800;
    let t5054 = t1170 * t5053;
    let t5056 = t1797 * t1176;
    let t5058 = t1166 * t1805;
    (t5047, t5048, t5049, t5051, t5053, t5054, t5056, t5058)
}
