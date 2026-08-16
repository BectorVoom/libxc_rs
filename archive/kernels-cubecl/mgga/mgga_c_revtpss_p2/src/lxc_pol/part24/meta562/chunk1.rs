//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1691/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1691<F: Float>(t11250: F, t11774: F, t11927: F, t15700: F, t15701: F, t15707: F, t16222: F, t19738: F, t19741: F, t23633: F, t23892: F, t23900: F, t23904: F, t23911: F, t23964: F, t3091: F, t3092: F, t3117: F, t43105: F, t6266: F, t78676: F, t78750: F, t78756: F, t78763: F, t78802: F, t79159: F, t88773: F, t88794: F) -> F {
    let t88800 = F::cast_from(0.57165357490759649296e-3_f64) * t78676 - F::cast_from(0.17149607247227894789e-2_f64) * t15707 * t23892 + F::cast_from(0.16937883700965822014e-2_f64) * t78750 + F::cast_from(0.34299214494455789577e-2_f64) * t11774 * t15701 * t23633 * t23911 + F::cast_from(0.28582678745379824648e-2_f64) * t15700 * t16222 * t88773 - F::cast_from(0.34299214494455789578e-2_f64) * t15700 * t15701 * t88773 + F::cast_from(0.51448821741683684368e-2_f64) * t11927 * t3117 * t23964 * t23911 + F::cast_from(0.57165357490759649296e-3_f64) * t3091 * t3092 * t79159 * t6266 + F::cast_from(0.34299214494455789578e-2_f64) * t19738 * t23900 - F::cast_from(0.17149607247227894789e-2_f64) * t19741 * t23904 + F::cast_from(0.19055119163586549765e-2_f64) * t78756 + F::cast_from(0.19055119163586549765e-2_f64) * t78763 + F::cast_from(0.51448821741683684368e-2_f64) * t43105 * t3117 * t88794 * t11250 + F::cast_from(0.34299214494455789578e-2_f64) * t78802;
    t88800
}
