//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1123/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1123(t1882: f64, t35107: f64, t35039: f64, t106623: f64, t12968: f64, t13140: f64, t13220: f64, t139675: f64, t139757: f64, t140087: f64, t140089: f64, t140094: f64, t140103: f64, t140112: f64, t140129: f64, t140338: f64, t140419: f64, t1901: f64, t2221: f64, t23443: f64, t23455: f64, t26520: f64, t26897: f64, t26928: f64, t27007: f64, t27334: f64, t27335: f64, t33034: f64, t33203: f64, t3446: f64, t3450: f64, t3455: f64, t3478: f64, t35063: f64, t35196: f64, t379: f64, t63180: f64, t925: f64, t9419: f64) -> f64 {
    let t148055 = t1882 * t35107;
    let t148057 = t1882 * t35039;
    let t148105 = t140087 / 9.0_f64 + t140089 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t148055 - 4.0_f64 / 9.0_f64 * t148057 + 2.0_f64 / 3.0_f64 * t140094 - 4.0_f64 * t1901 * t27334 * t27335 * t26520 - 2.0_f64 / 3.0_f64 * t1901 * t12968 * t33034 * t3450 - 2.0_f64 / 3.0_f64 * t1901 * t13140 * t140419 * t3455 - 4.0_f64 / 3.0_f64 * t1901 * t63180 * t33203 + 2.0_f64 / 9.0_f64 * t1901 * t23443 * t27007 + t1901 * t9419 * t35063 / 9.0_f64 + t1901 * t2221 * t140338 * t925 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t106623 * t26928 + t1901 * t139675 * t3446 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t140103 - 4.0_f64 / 9.0_f64 * t140112 - 4.0_f64 / 3.0_f64 * t1901 * t13140 * t23455 * t26897 - t140129 + 2.0_f64 * t1901 * t13140 * t139757 * t3478 - 2.0_f64 / 9.0_f64 * t1901 * t13220 * t35196 * t379;
    t148105
}
