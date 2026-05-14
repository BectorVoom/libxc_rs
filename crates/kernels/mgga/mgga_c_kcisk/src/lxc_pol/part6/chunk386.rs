//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 386/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk386<F: Float>(t1908: F, t2594: F, t1959: F, t2366: F, t1968: F, t1971: F, t2373: F, t2376: F, t2379: F, t1974: F, t1685: F, t2394: F, t1966: F, t1979: F, t1987: F, t2370: F, t2384: F, t2387: F, t2396: F, t240: F, t764: F) -> (F, F, F, F, F, F) {
    let t2595 = t1908 * t2594;
    let t2597 = -t1959 - 0.17123333333333333333e-1 * t2366;
    let t2604 = 0.3529725e1 * t2373 - t1968 - 0.516475e0 * t2366 + 0.6311625e0 * t2376 - t1971 - 0.104195e0 * t2379;
    let t2605 = t2604 * t1974;
    let t2609 = t2394 * t1685;
    let t2618 = -t2370 + t2384 + t240 * (-0.3109e-1 * t2597 * t764 + 1.0 * t1966 * t2605 + t2370 - t2384 - 0.19751789702565206229e-1 * t2387 + 0.58482233974552040708e0 * t1979 * t2609) + 0.19751789702565206229e-1 * t240 * t2387 - 0.58482233974552040708e0 * t1987 * t2396;
    (t2595, t2597, t2604, t2605, t2609, t2618)
}
