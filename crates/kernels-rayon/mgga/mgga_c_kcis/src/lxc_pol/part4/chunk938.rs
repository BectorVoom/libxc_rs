//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 938/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk938(t2593: f64, t62: f64, t2526: f64, t7617: f64, t153: f64, t7627: f64, t818: f64, t8755: f64, t9024: f64, t9026: f64, t9028: f64, t9031: f64, t9034: f64, t9036: f64, t9038: f64, t9040: f64, t9043: f64, t9048: f64, t9050: f64) -> (f64, f64, f64, f64) {
    let t9052 = t2593 * t62;
    let t9053 = t7617 * t2526;
    let t9054 = t9052 * t9053;
    let t9056 = t153 * t7627;
    let t9058 = t8755 * t818;
    let t9060 = 0.3375e1_f64 * t9024 - 0.2428125e1_f64 * t9026 + 0.225e1_f64 * t9028 - 0.485625e0_f64 * t9031 + 0.2428125e1_f64 * t9034 - 0.3375e1_f64 * t9036 - 0.97125e0_f64 * t9038 + 0.485625e0_f64 * t9040 + 0.1125e1_f64 * t9043 - 0.2428125e0_f64 * t9048 - 0.2428125e0_f64 * t9050 + 0.1125e1_f64 * t9054 - 0.45e1_f64 * t9056 + 0.12140625e0_f64 * t9058;
    (t9054, t9056, t9058, t9060)
}
