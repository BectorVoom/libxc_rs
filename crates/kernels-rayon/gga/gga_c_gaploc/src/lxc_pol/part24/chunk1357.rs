//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1357/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1357(t1570: f64, t188: f64, t3338: f64, t10215: f64, t1564: f64, t10122: f64, t10152: f64, t10337: f64, t10342: f64, t10485: f64, t10488: f64, t1265: f64, t1445: f64, t1457: f64, t1562: f64, t1572: f64, t31534: f64, t31719: f64, t34178: f64, t34181: f64, t34186: f64, t34189: f64, t34191: f64, t4540: f64, t4673: f64, t475: f64, t4762: f64, t4953: f64, t557: f64, t574: f64, t6740: f64, t6744: f64, t6820: f64) -> f64 {
    let t34195 = t188 * t1570 * t3338;
    let t34202 = t1564 * t10215;
    let t34207 = -0.21450293971110256001e1_f64 * t4540 * t1457 * t31719 + 0.95334639871601137784e0_f64 * t1572 * t4673 * t10152 - 0.14300195980740170668e1_f64 * t557 * t4673 * t10488 - 0.10725146985555128001e1_f64 * t10485 * t6820 - 0.46011511144704899612e1_f64 * t574 * t1445 * t10122 * t1265 + t34178 - t34181 + 0.71500979903700853338e0_f64 * t1572 * t1457 * t31534 - t34186 - t34189 - t34191 - 0.25025342966295298669e1_f64 * t10485 * t6740 + 0.42900587942220512003e1_f64 * t34195 * t6744 - 0.35750489951850426669e0_f64 * t10337 * t4762 - 0.13803453343411469884e2_f64 * t4953 * t10342 - 0.13803453343411469884e2_f64 * t1562 * t1445 * t34202 * t475;
    t34207
}
