//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3171/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3171<F: Float>(t15578: F, t4889: F, t11789: F, t1227: F, t248: F, t5979: F, t19051: F, t3523: F, t19080: F, t3572: F, t1174: F, t1177: F, t11825: F, t1213: F, t1214: F, t15581: F, t15584: F, t15587: F, t475: F, t6203: F, t63406: F, t65330: F, t65613: F, t65617: F, t65619: F, t65628: F, t65632: F) -> F {
    let t65637 = t4889 * t15578;
    let t65647 = t1227 * t248 * t11789 * t5979;
    let t65649 = t19051 * t3523;
    let t65651 = t19080 * t3572;
    let t65653 = -t65613 / F::cast_from(1728.0_f64) - t65617 / F::cast_from(3456.0_f64) - t65619 / F::cast_from(3456.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t11825 * t6203 + t1213 * t248 * t1214 * t65330 * t475 / F::cast_from(3072.0_f64) - t65628 / F::cast_from(1944.0_f64) + t65632 / F::cast_from(13824.0_f64) - t1174 * t1177 * t63406 / F::cast_from(12.0_f64) + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t65637 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t4889 * t15581 + t4889 * t15584 / F::cast_from(27.0_f64) + t4889 * t15587 / F::cast_from(9.0_f64) + t65647 / F::cast_from(20736.0_f64) - t65649 / F::cast_from(3456.0_f64) - t65651 / F::cast_from(216.0_f64);
    t65653
}
