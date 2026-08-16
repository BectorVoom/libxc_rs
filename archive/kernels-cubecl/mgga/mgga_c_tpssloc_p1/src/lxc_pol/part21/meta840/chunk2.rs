//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3015/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3015<F: Float>(t1049: F, t1058: F, t1060: F, t11065: F, t14488: F, t14578: F, t14606: F, t14622: F, t14640: F, t14645: F, t1610: F, t1625: F, t1630: F, t17959: F, t18080: F, t18103: F, t18161: F, t3120: F, t3200: F, t381: F, t4649: F, t4657: F, t4669: F, t4684: F, t47841: F, t50535: F, t5914: F, t5932: F, t62757: F) -> F {
    let t63133 = F::cast_from(2.0_f64) * t1049 * t1058 * t1060 * t17959 + F::cast_from(2.0_f64) * t1058 * t1060 * t14488 * t1625 + t1058 * t1060 * t3120 * t5914 + t1058 * t1060 * t381 * t62757 + F::cast_from(4.0_f64) * t1058 * t1060 * t4649 * t4657 - F::cast_from(12.0_f64) * t11065 * t18080 * t18103 - F::cast_from(2.0_f64) * t14622 * t3200 * t5932 - F::cast_from(2.0_f64) * t18161 * t3200 * t4684 + F::cast_from(12.0_f64) * t14578 * t47841 + F::cast_from(2.0_f64) * t14606 * t4669 + F::cast_from(2.0_f64) * t14640 * t1610 + F::cast_from(4.0_f64) * t14645 * t4669 + F::cast_from(2.0_f64) * t1630 * t50535;
    t63133
}
