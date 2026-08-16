//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 842/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk842(t1450: f64, t1562: f64, t1572: f64, t1584: f64, t2386: f64, t2859: f64, t2862: f64, t2869: f64, t4527: f64, t4771: f64, t4950: f64, t567: f64, t574: f64, t597: f64, t6740: f64, t6820: f64, t8118: f64, t8121: f64, t8126: f64, t8132: f64, t8136: f64, t8139: f64, t8142: f64, t8148: f64, t8152: f64, t8155: f64, t8158: f64, t8165: f64, t8168: f64, t8171: f64) -> f64 {
    let t8174 = 0.11502877786176224903e2_f64 * t597 * t8118 - 0.62115540045351614476e2_f64 * t1562 * t8121 + 0.27606906686822939767e2_f64 * t4527 * t8126 - 0.25025342966295298669e1_f64 * t2859 * t6740 - 0.92023022289409799224e1_f64 * t574 * t8132 + 0.43710935587469654631e2_f64 * t597 * t8136 + 0.46011511144704899612e1_f64 * t567 * t8139 + 0.23005755572352449806e1_f64 * t567 * t8142 - 0.46011511144704899612e1_f64 * t4771 * t2869 - 0.46011511144704899612e1_f64 * t1450 * t8148 - 0.69017266717057349418e1_f64 * t1562 * t8152 - 0.21450293971110256002e1_f64 * t8155 * t2386 - 0.21450293971110256002e1_f64 * t8158 * t2386 - 0.10725146985555128001e1_f64 * t2859 * t6820 + 0.14300195980740170668e1_f64 * t4950 * t2862 + 0.14300195980740170668e1_f64 * t1572 * t8165 + 0.71500979903700853338e0_f64 * t1572 * t8168 - 0.46011511144704899612e1_f64 * t1584 * t8171;
    t8174
}
