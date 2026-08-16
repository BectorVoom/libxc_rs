//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 878/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk878(t4260: f64, t5919: f64, t1552: f64, t2051: f64, t2055: f64, t4281: f64, t2039: f64, t4249: f64, t5627: f64, t584: f64, t583: f64, t1546: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5920 = t4260 * t5919;
    let t5922 = t2051 * t1552;
    let t5924 = t4281 * t2055;
    let t5926 = t4249 * t2039;
    let t5928 = t584 * t5627;
    let t5929 = t583 * t5928;
    let t5930 = t1546 * t5929;
    (t5920, t5922, t5924, t5926, t5929, t5930)
}
