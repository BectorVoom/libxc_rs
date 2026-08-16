//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 651/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk651(t2563: f64, t2567: f64, t1935: f64, t8973: f64, t9017: f64, t9021: f64, t9023: f64, t9025: f64, t9027: f64, t9031: f64, t9033: f64, t9037: f64, t9039: f64, t9041: f64) -> (f64, f64, f64) {
    let t9043 = t2567 * t2563;
    let t9044 = t1935 * t9043;
    let t9046 = t8973 / 256.0_f64 + t9017 / 16.0_f64 - t9021 / 72.0_f64 + t9023 / 128.0_f64 - t9025 / 3.0_f64 + t9027 / 12.0_f64 - t9031 / 16.0_f64 - t9033 / 8.0_f64 + t9037 / 24.0_f64 + t9039 / 24.0_f64 - t9041 / 96.0_f64 + t9044 / 3.0_f64;
    (t9043, t9044, t9046)
}
