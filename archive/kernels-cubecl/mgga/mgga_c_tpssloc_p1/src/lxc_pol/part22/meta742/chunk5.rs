//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2457/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2457<F: Float>(t21682: F, t225: F, t1009: F, t21480: F, t1057: F, t10482: F, t5866: F, t1022: F, t1049: F, t1058: F, t1060: F, t1061: F, t11059: F, t14618: F, t18083: F, t18100: F, t18111: F, t18138: F, t18162: F, t21594: F, t21618: F, t21637: F, t21643: F, t23508: F, t3180: F, t3186: F, t360: F, t43503: F, t43576: F, t43577: F, t4669: F, t50508: F, t50509: F, t5932: F) -> (F, F, F, F) {
    let t69871 = t21682 * t225;
    let t69923 = t21480 * t1009;
    let t69924 = t69923 * t1057;
    let t69935 = t10482 * t5866;
    let t69942 = -t1022 * t21637 * t23508 * t360 * t43503 + F::cast_from(24.0_f64) * t1022 * t21637 * t43576 * t43577 + F::cast_from(18.0_f64) * t1022 * t50508 * t50509 * t69935 + t1049 * t1058 * t1060 * t21594 + F::cast_from(18.0_f64) * t11059 * t18111 * t21643 + F::cast_from(12.0_f64) * t18138 * t3186 * t5932 + t1061 * t69924 + F::cast_from(6.0_f64) * t14618 * t18083 + F::cast_from(3.0_f64) * t18100 * t4669 + F::cast_from(3.0_f64) * t18162 * t4669 + F::cast_from(3.0_f64) * t21618 * t3180;
    (t69871, t69923, t69935, t69942)
}
