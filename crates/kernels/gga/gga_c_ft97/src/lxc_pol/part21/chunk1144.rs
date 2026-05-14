//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1144/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1144<F: Float>(t25542: F, t6414: F, t102364: F, t102366: F, t102369: F, t1286: F, t16169: F, t22935: F, t25558: F, t25577: F, t25602: F, t25615: F, t25616: F, t25849: F, t26117: F, t28: F, t29570: F, t29578: F, t29599: F, t3255: F, t3266: F, t38652: F, t4621: F, t492: F, t497: F, t5501: F, t5618: F, t6547: F, t8411: F, t8418: F) -> (F,) {
    let t116247 = t6414 * t25542;
    let t116249 = t102364 + t102366 + t102369 - 4.0 / 27.0 * t25577 * t25615 * t25616 * t16169 + 2.0 / 9.0 * t25558 * t25602 + 2.0 * t5501 * t8411 * t26117 * t3266 - t22935 * t29578 / 9.0 + t1286 * t28 * t29570 * t497 / 6.0 + t6414 * t25849 / 3.0 + t1286 * t28 * t5618 * t4621 / 6.0 - 24.0 * t8418 * t6547 * t3255 + 48.0 * t38652 * t29599 * t492 - t116247 / 9.0;
    (t116249,)
}
