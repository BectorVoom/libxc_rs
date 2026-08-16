//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 745/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk745<F: Float>(t1583: F, t30: F, t1468: F, t1940: F, t1963: F, t2403: F, t7091: F, t7750: F, t7783: F, t1659: F, t1972: F, t1656: F, t1665: F, t1671: F, t1675: F, t375: F, t7110: F, t7111: F, t7117: F, t7122: F, t7130: F, t7132: F) -> (F, F, F, F) {
    let t7787 = t30 * t1583;
    let t7794 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t7750 + t1940 * t7783 * t30 / F::cast_from(2.0_f64) - t1940 * t7091 * t7787 / F::cast_from(2.0_f64) + t1940 * t1963 * t1468 / F::cast_from(2.0_f64);
    let t7801 = t1659 * t1972;
    let t7810 = t7110 + t7111 * t1656 / F::cast_from(288.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t7801 * t375 - F::cast_from(0.42874018118069736972e-3_f64) * t7117 * t1665 + F::cast_from(0.42874018118069736972e-3_f64) * t7122 * t1671 + t7130 + F::cast_from(0.28582678745379824648e-3_f64) * t7132 * t1675;
    (t7787, t7794, t7801, t7810)
}
