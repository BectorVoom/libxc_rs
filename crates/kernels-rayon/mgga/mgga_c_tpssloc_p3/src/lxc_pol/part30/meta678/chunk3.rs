//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2123/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2123(t28030: f64, t6535: f64, t26114: f64, t7461: f64, t19994: f64, t24995: f64, t8945: f64, t1266: f64, t1393: f64, t1459: f64, t1774: f64, t1849: f64, t19450: f64, t19451: f64, t1976: f64, t20127: f64, t22461: f64, t24999: f64, t26098: f64, t26138: f64, t27993: f64, t28020: f64, t4037: f64, t4072: f64, t4077: f64, t5494: f64, t574: f64, t6517: f64, t652: f64, t6539: f64, t7670: f64, t96355: f64, t96358: f64, t96360: f64, t96361: f64, t96682: f64, t96732: f64) -> f64 {
    let t96738 = 2.0_f64 * t28030 * t6535;
    let t96740 = 4.0_f64 * t26114 * t7461;
    let t96746 = 6.0_f64 * t24995 * t8945 * t19994;
    let t96749 = -t19450 * t1976 + t96355 - t96358 - t96360 - 4.0_f64 * t96361 * t1459 - 4.0_f64 * t24999 * t4037 - 4.0_f64 * t24999 * t4077 - 2.0_f64 * t6517 * t20127 - 4.0_f64 * t652 * t7670 * t4072 - 2.0_f64 * t26098 * t1774 + t28020 * t1393 + (t96682 + t96732) * t574 - 2.0_f64 * t19451 * t6539 - t96738 - t96740 - t27993 * t1266 + 2.0_f64 * t26138 * t1849 + t96746 - 2.0_f64 * t22461 * t5494;
    t96749
}
