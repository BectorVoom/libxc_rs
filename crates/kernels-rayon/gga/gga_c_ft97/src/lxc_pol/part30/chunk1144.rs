//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1144/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1144(t34287: f64, t36071: f64, t1774: f64, t7087: f64, t7570: f64, t1526: f64, t7079: f64, t9483: f64, t1466: f64, t28804: f64, t1091: f64, t13616: f64, t142512: f64, t1477: f64, t2320: f64, t28517: f64, t28521: f64, t28525: f64, t28534: f64, t28720: f64, t28788: f64, t28796: f64, t28813: f64, t29414: f64, t34284: f64, t34296: f64, t36075: f64, t36080: f64, t3704: f64, t3746: f64, t461: f64, t6210: f64, t6216: f64, t6261: f64, t666: f64, t7150: f64, t7571: f64) -> f64 {
    let t153598 = t36071 * t34287;
    let t153611 = t7570 * t1774 * t7087;
    let t153617 = t1526 * t9483 * t7079;
    let t153619 = t1466 * t28804;
    let t153621 = -t142512 / 9.0_f64 - t6216 * t28813 / 9.0_f64 + t6216 * t28525 / 27.0_f64 - t6216 * t28517 / 9.0_f64 - t6216 * t28521 / 9.0_f64 - t29414 * t7150 * t7571 / 6.0_f64 - t7570 * t461 * t28720 / 6.0_f64 - t1466 * t3704 * t1477 * t3746 / 9.0_f64 + t6210 * t36075 / 18.0_f64 + t1466 * t666 * t6261 * t1091 / 18.0_f64 + t153598 / 18.0_f64 + t1526 * t13616 * t28788 / 6.0_f64 - t1526 * t2320 * t28796 / 12.0_f64 - t34284 * t36080 / 6.0_f64 - t36071 * t34296 / 6.0_f64 + t153611 / 18.0_f64 - t1526 * t2320 * t28534 / 12.0_f64 - t153617 / 36.0_f64 - t153619 / 9.0_f64;
    t153621
}
