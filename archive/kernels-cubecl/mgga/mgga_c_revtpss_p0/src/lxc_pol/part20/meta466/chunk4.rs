//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1783/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1783<F: Float>(t10146: F, t123: F, t3915: F, t676: F, t10008: F, t1358: F, t212: F, t689: F, t1359: F, t39501: F, t10171: F, t1424: F, t4071: F, t4076: F, t4077: F, t4131: F, t4132: F, t47521: F, t47525: F, t47527: F, t47531: F, t47534: F, t47537: F, t47540: F, t47546: F, t47550: F, t9657: F, t9659: F) -> F {
    let t47554 = t3915 * t123 * t676 * t10146;
    let t47558 = t689 * t212 * t10008 * t1358;
    let t47561 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t1359;
    let t47566 = -F::cast_from(0.13878983423218070567e-1_f64) * t47521 + F::cast_from(0.78059524315062264152e-1_f64) * t47525 + F::cast_from(0.15611904863012452831e0_f64) * t47527 + F::cast_from(0.23417857294518679245e0_f64) * t47531 + F::cast_from(0.39029762157531132075e-2_f64) * t47534 + F::cast_from(0.13170898365871023197e0_f64) * t47537 + F::cast_from(0.65854491829355115985e-1_f64) * t47540 - F::cast_from(0.39512695097613069592e1_f64) * t10171 * t4132 - F::cast_from(0.15805078039045227836e2_f64) * t4071 * t9659 + F::cast_from(0.39512695097613069591e1_f64) * t1424 * t4076 * t47546 - F::cast_from(0.11708928647259339623e0_f64) * t47550 - F::cast_from(0.39029762157531132076e-1_f64) * t47554 - F::cast_from(0.21951497276451705328e-1_f64) * t47558 + t47561 - F::cast_from(0.23707617058567841754e2_f64) * t1424 * t9657 * t4077 * t4131;
    t47566
}
