//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2202/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2202(t2517: f64, t2658: f64, t5392: f64, t12923: f64, t3966: f64, t4194: f64, t12924: f64, t16693: f64, t16616: f64, t2528: f64, t12932: f64, t4205: f64) -> (f64, f64, f64, f64, f64) {
    let t59013 = t2658 * t2517 * t5392;
    let t59022 = t4194 * t12923 * t3966;
    let t59024 = t16693 * t12924;
    let t59028 = t16616 * t2528;
    let t59032 = t4205 * t12932;
    (t59013, t59022, t59024, t59028, t59032)
}
