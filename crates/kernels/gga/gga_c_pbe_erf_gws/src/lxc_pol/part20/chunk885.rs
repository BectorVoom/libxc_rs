//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 885/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk885<F: Float>(t11054: F, t422: F, t4927: F, t639: F, t3473: F, t617: F, t1809: F, t1620: F, t5557: F, t7946: F, t3399: F, t583: F, t10561: F, t10564: F, t10567: F, t10570: F, t10573: F, t10576: F, t10579: F, t10581: F, t10583: F, t10585: F, t10588: F, t5083: F, t5360: F, t7269: F, t7272: F, t7278: F, t7819: F) -> (F, F, F, F, F, F) {
    let t11055 = t11054 * t422;
    let t11056 = t4927 * t11055;
    let t11058 = 8.0 / 45.0 * t639 * t11056;
    let t11059 = t3473 * t617;
    let t11060 = t1809 * t11059;
    let t11062 = 8.0 / 45.0 * t1620 * t11060;
    let t11063 = 8.0 / 135.0 * t5557;
    let t11064 = 16.0 / 135.0 * t7946;
    let t11065 = t3399 * t583;
    let t11066 = 8.0 / 45.0 * t11065;
    let t11082 = t5360 + 0.83962962962962962963e-3 * t5083 + 0.16792592592592592593e-2 * t7269 - 0.83962962962962962967e-3 * t7278 + t7819 + 0.2518888888888888889e-2 * t7272 - 0.41981481481481481483e-3 * t10581 + 0.20990740740740740742e-2 * t10561 - 0.75566666666666666669e-2 * t10564 - 0.5037777777777777778e-2 * t10567 + 0.12594444444444444445e-2 * t10583 + 0.11335e-1 * t10570 + 0.15113333333333333334e-1 * t10573 - 0.62972222222222222223e-3 * t10585 + 0.12594444444444444445e-2 * t10576 - 0.37783333333333333334e-2 * t10579 + 0.18891666666666666667e-2 * t10588;
    (t11058, t11062, t11063, t11064, t11066, t11082)
}
