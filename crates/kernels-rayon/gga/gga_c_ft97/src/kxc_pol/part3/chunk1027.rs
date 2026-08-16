//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1027/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1027(t19372: f64, t19428: f64, t19478: f64, t19531: f64, t19584: f64, t19791: f64, t19809: f64, t19880: f64, t1218: f64, t1253: f64, t18987: f64, t18989: f64, t18992: f64, t19308: f64, t301: f64, t317: f64, t4027: f64, t4135: f64, t4309: f64, t5207: f64, t5305: f64, t5422: f64, t830: f64, t880: f64) -> f64 {
    let t19883 = t19372 + t19428 + t19478 + t19531 + t19584 + t19791 + t19809 + t19880;
    let t19885 = -2.0_f64 * t1218 * t4309 - 2.0_f64 * t1253 * t4027 - 2.0_f64 * t1253 * t4135 - t18987 * t317 - t18989 * t317 - t18992 * t317 - t19308 * t317 - t19883 * t301 - t5207 * t880 - t5305 * t880 - t5422 * t830;
    t19885
}
