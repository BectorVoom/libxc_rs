//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1203/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1203<F: Float>(t1524: F, t1784: F, t3573: F, t3621: F, t6283: F, t1140: F, t6279: F, t1077: F, t1083: F, t1173: F, t1181: F, t13344: F, t1532: F, t16824: F, t16826: F, t16839: F, t16841: F, t16847: F, t16849: F, t1748: F, t336: F, t367: F) -> (F, F) {
    let t21901 = t1524 * t1524;
    let t21906 = t3573 * t1784;
    let t21908 = t3621 * t6283;
    let t21910 = t1140 * t6279;
    let t21919 = F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t1181 * t1532 * t1748 * t1077 + t367 * t336 * t1083 * t21901 / F::new(24.0) + F::new(35.0) / F::new(216.0) * t21906 - F::new(7.0) / F::new(24.0) * t21908 - F::new(7.0) / F::new(72.0) * t21910 + F::new(7.0) / F::new(36.0) * t16824 + F::new(7.0) / F::new(72.0) * t16826 - F::new(7.0) / F::new(36.0) * t16839 - F::new(7.0) / F::new(72.0) * t16841 - F::new(7.0) / F::new(36.0) * t16847 - F::new(7.0) / F::new(72.0) * t16849 - F::cast_from(0.80031500487063509016e-2_f64) * t13344;
    (t21901, t21919)
}
