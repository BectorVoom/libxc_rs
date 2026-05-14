//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1301/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1301<F: Float>(t1882: F, t30461: F, t30552: F, t30524: F, t30357: F, t604: F, t2178: F, t6615: F, t106351: F, t106361: F, t106573: F, t119447: F, t119665: F, t12968: F, t13140: F, t144: F, t167: F, t16997: F, t1901: F, t2185: F, t2210: F, t23470: F, t26883: F, t26897: F, t27016: F, t3478: F, t3483: F, t3578: F, t379: F, t446: F, t4823: F, t574: F, t63180: F, t95696: F) -> (F,) {
    let t120413 = t1882 * t30461;
    let t120422 = t1882 * t30552;
    let t120438 = t1882 * t30524;
    let t120440 = t604 * t30357;
    let t120449 = t2178 * t6615;
    let t120458 = -4.0 / 9.0 * t120413 + 2.0 / 9.0 * t1901 * t23470 * t16997 + 8.0 / 81.0 * t106351 + 2.0 / 9.0 * t1901 * t95696 * t4823 + 2.0 / 27.0 * t120422 + 2.0 / 3.0 * t446 * t574 * t3578 * t26897 - t106361 + 2.0 / 3.0 * t446 * t574 * t3578 * t26883 - 4.0 / 3.0 * t1901 * t63180 * t27016 + 2.0 / 3.0 * t446 * t144 * t119447 + 2.0 / 9.0 * t120438 + t1901 * t2210 * t120440 * t379 / 9.0 - 4.0 / 3.0 * t1901 * t12968 * t106573 * t3478 - 4.0 / 3.0 * t1901 * t13140 * t120449 * t3483 + 2.0 / 3.0 * t446 * t2185 * t167 * t119665;
    (t120458,)
}
