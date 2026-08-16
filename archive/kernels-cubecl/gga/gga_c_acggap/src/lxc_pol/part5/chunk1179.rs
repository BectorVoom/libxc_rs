//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1179/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1179<F: Float>(t1137: F, t6310: F, t1140: F, t6314: F, t3382: F, t5991: F, t1165: F, t1173: F, t1181: F, t16546: F, t16551: F, t1889: F, t3169: F, t3282: F, t335: F, t360: F, t3616: F, t367: F, t372: F, t4450: F, t5852: F, t5853: F, t5922: F, t6309: F, t6313: F, t6319: F, t6375: F) -> F {
    let t21390 = t1137 * t6310;
    let t21401 = t1140 * t6314;
    let t21414 = t3382 * t5991;
    let t21421 = -t335 * t3282 * t6309 / F::cast_from(12.0_f64) + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t21390 - t367 * t3282 * t6313 / F::cast_from(8.0_f64) + t367 * t3282 * t6319 / F::cast_from(12.0_f64) - t3616 * t3282 * t6375 / F::cast_from(2.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t21401 + F::cast_from(0.34299214494455789578e-1_f64) * t16546 - F::cast_from(0.51448821741683684367e-2_f64) * t4450 * t1181 * t5852 * t5853 * t360 + F::cast_from(0.51448821741683684367e-2_f64) * t4450 * t1165 * t5922 * t5853 * t372 + F::cast_from(0.34299214494455789578e-2_f64) * t21414 + F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t1181 * t1889 * t3169 + F::cast_from(0.42874018118069736972e-3_f64) * t16551;
    t21421
}
