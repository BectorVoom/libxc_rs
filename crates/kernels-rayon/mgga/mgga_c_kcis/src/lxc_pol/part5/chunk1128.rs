//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1128/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1128(t6366: f64, t949: f64, t2986: f64, t4740: f64, t5250: f64, t1226: f64, t6428: f64, t6406: f64, t9825: f64, t4764: f64, t45: f64, t6400: f64) -> (f64, f64, f64, f64, f64) {
    let t18997 = t6366 * t949;
    let t18999 = 6.0_f64 * t2986 * t18997;
    let t19006 = t4740 * t5250;
    let t19011 = t6428 * t1226;
    let t19018 = t9825 * t6406;
    let t19019 = t19018 * t4764;
    let t19022 = t45 * t6400;
    (t18999, t19006, t19011, t19019, t19022)
}
