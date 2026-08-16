//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1205/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1205(t113372: f64, t113392: f64, t113438: f64, t1615: f64, t1618: f64, t1622: f64, t1935: f64, t23483: f64, t23535: f64, t23564: f64, t25608: f64, t25645: f64, t25652: f64, t25654: f64, t25660: f64, t30816: f64, t30817: f64, t30821: f64, t30829: f64, t32948: f64, t32951: f64, t4652: f64, t6730: f64, t6753: f64, t7574: f64) -> f64 {
    let t119303 = 0.40372756094140390856e-3_f64 * t113372 - 0.40372756094140390856e-3_f64 * t1935 * t25608 * t30816 + t113392 * t1618 / 1536.0_f64 + t30829 * t4652 / 1536.0_f64 + t113438 * t1622 / 2304.0_f64 - 0.40372756094140390856e-3_f64 * t25645 * t30821 - 0.40372756094140390856e-3_f64 * t23564 * t32951 + 0.80745512188280781712e-3_f64 * t25652 * t23535 * t1615 * t25654 - 0.40372756094140390856e-3_f64 * t7574 * t30817 - 0.40372756094140390856e-3_f64 * t6730 * t32948 - 0.32298204875312312685e-2_f64 * t23483 * t32951 - 0.40372756094140390856e-3_f64 * t25652 * t6753 * t1615 * t25660;
    t119303
}
