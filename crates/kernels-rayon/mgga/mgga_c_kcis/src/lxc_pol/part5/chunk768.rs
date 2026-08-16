//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 768/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk768(t4261: f64, t5880: f64, t4260: f64, t1551: f64, t2061: f64, t578: f64, t143: f64, t5623: f64, t1317: f64, t562: f64, t543: f64, t1478: f64, t1483: f64, t1507: f64, t1991: f64, t1995: f64, t2018: f64, t4202: f64, t545: f64, t5459: f64, t5464: f64, t5482: f64, t5494: f64, t5499: f64, t5527: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5932 = t4261 * t5880;
    let t5933 = t4260 * t5932;
    let t5935 = t2061 * t1551;
    let t5936 = t578 * t5935;
    let t5938 = t5623 * t143;
    let t5947 = t562 * t1317;
    let t5958 = t562 * t543;
    let t5963 = 0.619125e-2_f64 * t5938 * t545 + 0.9286875e-2_f64 * t2018 * t1478 - 0.619125e-2_f64 * t2018 * t1483 + 0.9286875e-2_f64 * t1507 * t1991 + 0.46434375e-2_f64 * t5947 * t5459 - 0.9286875e-2_f64 * t4202 * t5464 + 0.9286875e-2_f64 * t562 * t5482 - 0.619125e-2_f64 * t1507 * t1995 - 0.9286875e-2_f64 * t4202 * t5494 + 0.123825e-1_f64 * t5958 * t5499 - 0.619125e-2_f64 * t562 * t5527;
    (t5932, t5933, t5935, t5936, t5938, t5947, t5958, t5963)
}
