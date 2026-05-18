//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 840/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk840<F: Float>(t1450: F, t1562: F, t1572: F, t1584: F, t2386: F, t2859: F, t2862: F, t2869: F, t4527: F, t4771: F, t4950: F, t567: F, t574: F, t597: F, t6740: F, t6820: F, t8118: F, t8121: F, t8126: F, t8132: F, t8136: F, t8139: F, t8142: F, t8148: F, t8152: F, t8155: F, t8158: F, t8165: F, t8168: F, t8171: F) -> F {
    let t8174 = F::new(0.11502877786176224903e2) * t597 * t8118 - F::new(0.62115540045351614476e2) * t1562 * t8121 + F::new(0.27606906686822939767e2) * t4527 * t8126 - F::new(0.25025342966295298669e1) * t2859 * t6740 - F::new(0.92023022289409799224e1) * t574 * t8132 + F::new(0.43710935587469654631e2) * t597 * t8136 + F::new(0.46011511144704899612e1) * t567 * t8139 + F::new(0.23005755572352449806e1) * t567 * t8142 - F::new(0.46011511144704899612e1) * t4771 * t2869 - F::new(0.46011511144704899612e1) * t1450 * t8148 - F::new(0.69017266717057349418e1) * t1562 * t8152 - F::new(0.21450293971110256002e1) * t8155 * t2386 - F::new(0.21450293971110256002e1) * t8158 * t2386 - F::new(0.10725146985555128001e1) * t2859 * t6820 + F::new(0.14300195980740170668e1) * t4950 * t2862 + F::new(0.14300195980740170668e1) * t1572 * t8165 + F::new(0.71500979903700853338e0) * t1572 * t8168 - F::new(0.46011511144704899612e1) * t1584 * t8171;
    t8174
}
