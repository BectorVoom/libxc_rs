//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 922/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk922(t10668: f64, t10867: f64, t13682: f64, t13685: f64, t1445: f64, t1645: f64, t1890: f64, t1966: f64, t1998: f64, t2197: f64, t3025: f64, t33436: f64, t41244: f64, t43812: f64, t43817: f64, t44888: f64, t44939: f64, t45627: f64, t45630: f64, t45633: f64, t45636: f64, t45639: f64, t45648: f64, t45653: f64, t45658: f64, t45663: f64, t45667: f64, t5241: f64, t5640: f64, t590: f64, t701: f64, t8638: f64) -> f64 {
    let t45672 = -0.63904876589867916126e-1_f64 * t41244 - 0.23005755572352449806e1_f64 * t1998 * t1445 * t44939 * t701 - t45627 + t45630 - t45633 - t45636 + t45639 - 0.21450293971110256002e1_f64 * t8638 * t13682 - 0.21450293971110256002e1_f64 * t3025 * t1645 * t10668 + 0.23005755572352449806e2_f64 * t2197 * t13685 + t45648 - 0.59584149919750711116e-1_f64 * t43812 - 0.59584149919750711116e-1_f64 * t43817 + t45653 - 0.50050685932590597338e1_f64 * t10867 * t33436 + t45658 + 0.30674340763136599742e1_f64 * t5640 * t5241 * t44888 * t590 + t45663 - t45667 - 0.51123901271894332902e1_f64 * t1966 * t1890 * t44888 * t590;
    t45672
}
