//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1356/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1356<F: Float>(t1570: F, t188: F, t3338: F, t10215: F, t1564: F, t10122: F, t10152: F, t10337: F, t10342: F, t10485: F, t10488: F, t1265: F, t1445: F, t1457: F, t1562: F, t1572: F, t31534: F, t31719: F, t34178: F, t34181: F, t34186: F, t34189: F, t34191: F, t4540: F, t4673: F, t475: F, t4762: F, t4953: F, t557: F, t574: F, t6740: F, t6744: F, t6820: F) -> F {
    let t34195 = t188 * t1570 * t3338;
    let t34202 = t1564 * t10215;
    let t34207 = -F::new(0.21450293971110256001e1) * t4540 * t1457 * t31719 + F::new(0.95334639871601137784e0) * t1572 * t4673 * t10152 - F::new(0.14300195980740170668e1) * t557 * t4673 * t10488 - F::new(0.10725146985555128001e1) * t10485 * t6820 - F::new(0.46011511144704899612e1) * t574 * t1445 * t10122 * t1265 + t34178 - t34181 + F::new(0.71500979903700853338e0) * t1572 * t1457 * t31534 - t34186 - t34189 - t34191 - F::new(0.25025342966295298669e1) * t10485 * t6740 + F::new(0.42900587942220512003e1) * t34195 * t6744 - F::new(0.35750489951850426669e0) * t10337 * t4762 - F::new(0.13803453343411469884e2) * t4953 * t10342 - F::new(0.13803453343411469884e2) * t1562 * t1445 * t34202 * t475;
    t34207
}
