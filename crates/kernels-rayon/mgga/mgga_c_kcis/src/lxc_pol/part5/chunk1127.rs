//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1127/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1127(t13872: f64, t4723: f64, t18866: f64, t18868: f64, t18870: f64, t18872: f64, t18874: f64, t18947: f64, t18949: f64, t18965: f64, t18970: f64, t18973: f64, t18976: f64, t18980: f64, t18983: f64, t18987: f64, t18989: f64, t18993: f64, t45: f64, t960: f64) -> (f64, f64) {
    let t18995 = 0.32163648644302209644e2_f64 * t13872 * t4723;
    let t18996 = t18866 + t18868 + t18870 - t18872 + t18874 + t18947 + t18949 + 0.19751789702565206229e-1_f64 * t45 * t18965 - t18970 - t18973 - t18976 + t18980 + t18983 + t18987 - 0.17315755899375863299e2_f64 * t960 * t18989 - t18993 + t18995;
    (t18995, t18996)
}
