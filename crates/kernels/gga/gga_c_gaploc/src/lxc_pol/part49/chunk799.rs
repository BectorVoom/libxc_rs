//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 799/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk799<F: Float>(t12256: F, t3470: F, t12207: F, t955: F, t13121: F, t13124: F, t13127: F, t13132: F, t13134: F, t13138: F, t13140: F, t13144: F, t13147: F, t13152: F, t13156: F, t13160: F, t13163: F) -> F {
    let t13904 = t12256 * t3470;
    let t13906 = t955 * t12207;
    let t13912 = -F::cast_from(0.10725146985555128001e1_f64) * t13904 + F::cast_from(0.35750489951850426669e0_f64) * t13906 + F::cast_from(0.14896037479937677779e-1_f64) * t13121 + t13124 - F::cast_from(0.46011511144704899612e1_f64) * t13127 - t13132 + F::cast_from(0.11502877786176224903e2_f64) * t13134 + t13138 + t13140 + t13144 - F::cast_from(0.14896037479937677779e-1_f64) * t13147 - t13152 + t13156 - t13160 - t13163;
    t13912
}
