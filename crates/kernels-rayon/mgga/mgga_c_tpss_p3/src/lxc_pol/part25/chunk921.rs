//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 921/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk921(t14: f64, t563: f64, t498: f64, t1193: f64, t8038: f64, t2206: f64, t3178: f64, t1184: f64, t3214: f64, t3305: f64, t8027: f64, t1170: f64, t3280: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10021 = t14 * t563;
    let t10022 = t10021 * t498;
    let t10028 = 0.10254018858216406658e4_f64 * t1193 * t8038;
    let t10029 = t3178 * t2206;
    let t10031 = t3214 * t1184;
    let t10033 = t3305 * t1184;
    let t10038 = 0.35089341735807877242e1_f64 * t1193 * t8027;
    let t10039 = t1170 * t3280;
    (t10022, t10028, t10029, t10031, t10033, t10038, t10039)
}
