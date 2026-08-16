//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 616/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk616(t2039: f64, t4249: f64, t5627: f64, t584: f64, t583: f64, t1546: f64, t4261: f64, t5880: f64, t4260: f64, t1551: f64, t2061: f64, t578: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5926 = t4249 * t2039;
    let t5928 = t584 * t5627;
    let t5929 = t583 * t5928;
    let t5930 = t1546 * t5929;
    let t5932 = t4261 * t5880;
    let t5933 = t4260 * t5932;
    let t5935 = t2061 * t1551;
    let t5936 = t578 * t5935;
    (t5926, t5929, t5930, t5932, t5933, t5935, t5936)
}
