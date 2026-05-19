//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 563/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk563<F: Float>(t2089: F, t3234: F, t723: F, t1445: F, t2004: F, t2087: F, t2103: F, t2638: F, t5974: F, t6141: F, t780: F, t807: F, t9942: F, t9946: F, t9947: F, t9950: F, t9955: F, t9958: F, t9961: F, t9966: F, t9969: F, t9972: F) -> F {
    let t9975 = t2089 * t3234;
    let t9976 = t9975 * t723;
    let t9977 = t1445 * t9976;
    let t9980 = -t9942 - t9946 + F::cast_from(0.23005755572352449806e1_f64) * t807 * t9947 + F::cast_from(0.35750489951850426669e0_f64) * t2004 * t9950 + F::cast_from(0.46011511144704899612e1_f64) * t807 * t9955 - F::cast_from(0.71500979903700853338e0_f64) * t6141 * t9958 + F::cast_from(0.14300195980740170668e1_f64) * t2103 * t9961 + F::cast_from(0.35750489951850426669e0_f64) * t780 * t9966 + F::cast_from(0.10725146985555128001e1_f64) * t9969 * t5974 - F::cast_from(0.21450293971110256002e1_f64) * t2638 * t9972 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t9977;
    t9980
}
