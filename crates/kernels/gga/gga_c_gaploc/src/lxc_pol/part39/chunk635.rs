//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 635/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk635<F: Float>(t3377: F, t8155: F, t8158: F, t10465: F, t10468: F, t10472: F, t10476: F, t10479: F, t10480: F, t10484: F, t10485: F, t10489: F, t10492: F, t10497: F, t1456: F, t2386: F, t536: F, t574: F, t597: F, t9540: F, t9546: F) -> (F, F, F) {
    let t10501 = F::cast_from(0.10725146985555128001e1_f64) * t8155 * t3377;
    let t10503 = F::cast_from(0.10725146985555128001e1_f64) * t8158 * t3377;
    let t10504 = t10465 - t10468 - t10472 + t10476 - t9540 + t9546 + t10479 + F::cast_from(0.35750489951850426669e0_f64) * t1456 * t10480 + t10484 - F::cast_from(0.10725146985555128001e1_f64) * t10485 * t2386 - F::cast_from(0.46011511144704899612e1_f64) * t574 * t10489 + F::cast_from(0.11502877786176224903e2_f64) * t597 * t10492 + F::cast_from(0.35750489951850426669e0_f64) * t536 * t10497 - t10501 - t10503;
    (t10501, t10503, t10504)
}
