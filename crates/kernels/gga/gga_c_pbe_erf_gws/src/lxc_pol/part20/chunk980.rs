//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 980/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk980<F: Float>(t5557: F, t7946: F, t3399: F, t583: F, t10561: F, t10564: F, t10567: F, t10570: F, t10573: F, t10576: F, t10579: F, t10581: F, t10583: F, t10585: F, t10588: F, t5083: F, t5360: F, t7269: F, t7272: F, t7278: F, t7819: F) -> (F, F, F, F) {
    let t11063 = F::new(8.0) / F::new(135.0) * t5557;
    let t11064 = F::new(16.0) / F::new(135.0) * t7946;
    let t11065 = t3399 * t583;
    let t11066 = F::new(8.0) / F::new(45.0) * t11065;
    let t11082 = t5360 + F::new(0.83962962962962962963e-3) * t5083 + F::new(0.16792592592592592593e-2) * t7269 - F::new(0.83962962962962962967e-3) * t7278 + t7819 + F::new(0.2518888888888888889e-2) * t7272 - F::new(0.41981481481481481483e-3) * t10581 + F::new(0.20990740740740740742e-2) * t10561 - F::new(0.75566666666666666669e-2) * t10564 - F::new(0.5037777777777777778e-2) * t10567 + F::new(0.12594444444444444445e-2) * t10583 + F::new(0.11335e-1) * t10570 + F::new(0.15113333333333333334e-1) * t10573 - F::new(0.62972222222222222223e-3) * t10585 + F::new(0.12594444444444444445e-2) * t10576 - F::new(0.37783333333333333334e-2) * t10579 + F::new(0.18891666666666666667e-2) * t10588;
    (t11063, t11064, t11066, t11082)
}
