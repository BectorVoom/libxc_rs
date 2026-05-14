//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1132/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1132<F: Float>(t401: F, t4417: F, t101139: F, t101173: F, t101247: F, t101249: F, t115815: F, t12477: F, t1300: F, t15724: F, t15786: F, t15879: F, t1669: F, t1701: F, t1742: F, t2248: F, t22541: F, t22583: F, t22584: F, t22585: F, t22591: F, t22652: F, t22696: F, t22826: F, t29523: F, t3052: F, t3057: F, t379: F, t398: F, t4467: F, t4474: F, t4491: F, t51: F, t5522: F, t5570: F, t58935: F, t6: F, t925: F, t92596: F, t92612: F, t92686: F, t92835: F, t930: F, t93268: F) -> (F, F) {
    let t115885 = t4417 * t401;
    let t115904 = -2.0 * t1669 * t5522 * t15879 - 2.0 * t22696 * t29523 + 0.14187468528806584362e-2 * t92835 + 0.2370952259137005195e-1 * t92686 * t4474 * t6 * t51 * t398 - 0.35625083901748972663e-8 * t58935 * t92612 - 0.88910709717637694816e-2 * t92596 * t22591 * t115815 * t401 + 0.23254900946437792e-1 * t22826 * t15786 - 0.46509801892875584e-1 * t93268 * t15724 - 0.11854761295685025975e-1 * t1300 * t1701 * t22652 * t4491 - t101139 + 0.7423383944657264111e-4 * t22583 * t22585 * t4467 * t379 + 0.25537443351851851852e-1 * t22541 * t5570 * t1742 * t115885 + 0.14846767889314528222e-3 * t22583 * t22585 * t3057 * t925 - 0.34526011664076264185e-5 * t101247 * t101249 * t12477 * t925 + 0.29693535778629056444e-3 * t22583 * t2248 * t22584 * t930 * t3052 - 0.98978452595430188147e-4 * t101173;
    (t115885, t115904)
}
