//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 963/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk963<F: Float>(t1: F, t3338: F, t106: F, t192: F, t3377: F, t8155: F, t8158: F, t10465: F, t10468: F, t10472: F, t10476: F, t10479: F, t10480: F, t10484: F, t10485: F, t10489: F, t10492: F, t1456: F, t2386: F, t536: F, t574: F, t597: F, t9540: F, t9546: F) -> (F, F, F, F) {
    let t10495 = t3338 * t1;
    let t10496 = t10495 * t106;
    let t10497 = t10496 * t192;
    let t10501 = F::new(0.10725146985555128001e1) * t8155 * t3377;
    let t10503 = F::new(0.10725146985555128001e1) * t8158 * t3377;
    let t10504 = t10465 - t10468 - t10472 + t10476 - t9540 + t9546 + t10479 + F::new(0.35750489951850426669e0) * t1456 * t10480 + t10484 - F::new(0.10725146985555128001e1) * t10485 * t2386 - F::new(0.46011511144704899612e1) * t574 * t10489 + F::new(0.11502877786176224903e2) * t597 * t10492 + F::new(0.35750489951850426669e0) * t536 * t10497 - t10501 - t10503;
    (t10495, t10496, t10497, t10504)
}
