//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1175/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1175<F: Float>(t17040: F, t17044: F, t17054: F, t17056: F, t17088: F, t17089: F, t17096: F, t17098: F, t17100: F, t1753: F, t179: F, t20405: F, t20407: F, t20409: F, t20419: F, t20427: F, t20436: F, t2592: F, t2593: F, t5244: F, t5279: F, t568: F, t6896: F, t6939: F, t6961: F) -> F {
    let t20438 = F::cast_from(0.30011812682648815881e-2_f64) * t20405 + F::cast_from(0.34013387707001991331e0_f64) * t20407 + F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t20409 - F::cast_from(0.12004725073059526352e-1_f64) * t17040 + F::cast_from(0.60023625365297631762e-1_f64) * t17044 + F::cast_from(0.13605355082800796533e0_f64) * t17054 - F::cast_from(0.12004725073059526352e-1_f64) * t17056 - F::cast_from(0.12862205435420921092e-1_f64) * t5279 * t179 * t6961 * t6939 + F::cast_from(0.38586616306262763276e-2_f64) * t2592 * t179 * t20419 + t17088 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t17089 + F::cast_from(0.45351183609335988443e0_f64) * t17096 - F::cast_from(0.68026775414003982663e-1_f64) * t17098 + F::cast_from(0.34013387707001991332e0_f64) * t17100 - F::cast_from(0.38586616306262763276e-2_f64) * t6896 * t179 * t20427 - F::cast_from(0.51448821741683684367e-2_f64) * t5244 * t179 * t2593 * t1753 * t568 + F::cast_from(0.48018900292238105409e-1_f64) * t20436;
    t20438
}
