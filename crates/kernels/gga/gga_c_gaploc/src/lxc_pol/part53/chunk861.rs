//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 861/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk861<F: Float>(t8411: F, t9327: F, t10556: F, t1415: F, t9321: F, t34600: F, t544: F, t9287: F, t34604: F, t10532: F, t10533: F, t41726: F) -> (F, F, F, F, F) {
    let t42356 = F::cast_from(0.10725146985555128001e1_f64) * t8411 * t9327;
    let t42359 = F::cast_from(0.42900587942220512003e1_f64) * t1415 * t10556 * t9321;
    let t42366 = t544 * t34600 * t9287;
    let t42367 = F::cast_from(0.29792074959875355558e-1_f64) * t42366;
    let t42369 = t544 * t34604 * t9287;
    let t42370 = F::cast_from(0.29792074959875355558e-1_f64) * t42369;
    let t42373 = F::cast_from(0.38649669361552115674e3_f64) * t10532 * t10533 * t41726;
    (t42356, t42359, t42367, t42370, t42373)
}
