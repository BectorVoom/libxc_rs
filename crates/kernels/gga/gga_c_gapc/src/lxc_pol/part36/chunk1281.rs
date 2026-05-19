//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1281/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1281<F: Float>(t35552: F, t35555: F, t35557: F, t35559: F, t35562: F, t35564: F, t35566: F, t35570: F, t35572: F, t35575: F, t35578: F, t35580: F, t35584: F) -> F {
    let t37430 = F::cast_from(0.72415202344614669852e-6_f64) * t35552 + F::cast_from(0.72415202344614669852e-6_f64) * t35555 - F::cast_from(0.23897016773722841052e-3_f64) * t35557 - F::cast_from(0.1502132680635123635e-4_f64) * t35559 - F::cast_from(0.6180203028898794384e-4_f64) * t35562 - F::cast_from(0.32055487582266971205e-4_f64) * t35564 - F::cast_from(0.1619645688367173282e-3_f64) * t35566 + F::cast_from(0.1619645688367173282e-3_f64) * t35570 + F::cast_from(0.7828287493774670863e-3_f64) * t35572 - F::cast_from(0.4858937065101519846e-3_f64) * t35575 - F::cast_from(0.2429468532550759923e-3_f64) * t35578 - F::cast_from(0.4858937065101519846e-3_f64) * t35580 + F::cast_from(0.4858937065101519846e-3_f64) * t35584;
    t37430
}
