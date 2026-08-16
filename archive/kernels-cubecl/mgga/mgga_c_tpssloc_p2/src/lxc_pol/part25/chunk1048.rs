//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1048/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1048<F: Float>(t12030: F, t12033: F, t12444: F, t1375: F, t1386: F, t2092: F, t22639: F, t22650: F, t24064: F, t24071: F, t24082: F, t24088: F, t24092: F, t24095: F, t3758: F, t3882: F, t3889: F, t3912: F, t568: F, t7194: F, t7199: F, t7214: F) -> F {
    let t24098 = t24064 * t568 + F::cast_from(4.0_f64) * t3758 * t7199 + F::cast_from(4.0_f64) * t3882 * t7199 + F::cast_from(0.6579736267392905746e-1_f64) * t22639 - t24071 - F::cast_from(2.0_f64) * t3758 * t7214 - F::cast_from(2.0_f64) * t3882 * t7214 - t7194 * t3912 - t12030 * t2092 - t12033 * t2092 + F::cast_from(0.16449340668482264365e-1_f64) * t22650 - F::cast_from(2.0_f64) * t12444 * t2092 - F::cast_from(2.0_f64) * t24082 * t1386 + F::cast_from(2.0_f64) * t7194 * t3889 + F::cast_from(2.0_f64) * t1375 * t24088 - F::cast_from(6.0_f64) * t1375 * t24092 - F::cast_from(2.0_f64) * t24095 * t1386;
    t24098
}
