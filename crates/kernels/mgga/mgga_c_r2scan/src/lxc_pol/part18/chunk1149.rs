//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1149/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1149<F: Float>(t322: F, t42547: F, t1020: F, t1083: F, t1085: F, t11979: F, t11981: F, t11983: F, t11985: F, t2410: F, t2956: F, t3388: F, t3390: F, t3394: F, t3398: F, t3650: F, t3652: F, t3656: F, t3660: F, t9707: F) -> (F, F) {
    let t332 = F::new(0.25e1) < t322;
    let t42616 = piecewise3::<F>(t332, F::new(0.0), t42547);
    let t42646 = -F::new(0.64e0) * t42616 - F::cast_from(0.18428227254588e2_f64) * t3650 * t2410 - F::cast_from(0.18428227254588e2_f64) * t11979 * t1020 - F::cast_from(0.18428227254588e2_f64) * t11981 * t1020 - F::cast_from(0.18428227254588e2_f64) * t3652 * t2410 - F::cast_from(0.9214113627294e1_f64) * t3388 * t2956 - F::cast_from(0.9214113627294e1_f64) * t3390 * t2956 - F::cast_from(0.9214113627294e1_f64) * t1083 * t9707 + F::cast_from(0.734774460522e2_f64) * t11983 * t1020 + F::cast_from(0.734774460522e2_f64) * t3656 * t2410 + F::cast_from(0.367387230261e2_f64) * t3394 * t2956 + F::cast_from(0.367387230261e2_f64) * t1085 * t9707 - F::cast_from(0.7662840944824e2_f64) * t11985 * t1020 - F::cast_from(0.7662840944824e2_f64) * t3660 * t2410 - F::cast_from(0.3831420472412e2_f64) * t3398 * t2956;
    (t42616, t42646)
}
