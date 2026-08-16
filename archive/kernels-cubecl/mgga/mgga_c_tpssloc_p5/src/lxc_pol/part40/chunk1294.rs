//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1294/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1294<F: Float>(t109: F, t111379: F, t111413: F, t1268: F, t12725: F, t19451: F, t19456: F, t2181: F, t26114: F, t26179: F, t28002: F, t28030: F, t30195: F, t30201: F, t30203: F, t30209: F, t4028: F, t574: F, t7458: F, t75560: F, t8124: F, t8144: F, t8221: F, t8231: F, t8237: F, t96683: F, t96709: F, t97933: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t111415 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t111379 + t111413);
    let t111457 = F::cast_from(2.0_f64) * t111415 * t1268 * t574 - F::cast_from(4.0_f64) * t12725 * t8221 - F::cast_from(2.0_f64) * t19451 * t8124 - F::cast_from(4.0_f64) * t19456 * t8221 - F::cast_from(4.0_f64) * t19456 * t8231 + F::cast_from(4.0_f64) * t19456 * t8237 - F::cast_from(2.0_f64) * t2181 * t75560 - F::cast_from(4.0_f64) * t2181 * t96683 - F::cast_from(2.0_f64) * t2181 * t96709 - F::cast_from(2.0_f64) * t2181 * t97933 - F::cast_from(4.0_f64) * t26114 * t8221 - F::cast_from(4.0_f64) * t26179 * t8221 - F::cast_from(4.0_f64) * t28002 * t8144 - F::cast_from(2.0_f64) * t28030 * t8124 - F::cast_from(2.0_f64) * t28030 * t8144 - F::cast_from(4.0_f64) * t30195 * t4028 + F::cast_from(4.0_f64) * t30201 * t4028 - F::cast_from(4.0_f64) * t30203 * t4028 - F::cast_from(4.0_f64) * t30209 * t4028 - F::cast_from(4.0_f64) * t30209 * t7458;
    (t111415, t111457)
}
