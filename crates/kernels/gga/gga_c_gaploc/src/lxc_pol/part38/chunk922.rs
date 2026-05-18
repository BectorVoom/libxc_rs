//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 922/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk922<F: Float>(t10668: F, t10867: F, t13682: F, t13685: F, t1445: F, t1645: F, t1890: F, t1966: F, t1998: F, t2197: F, t3025: F, t33436: F, t41244: F, t43812: F, t43817: F, t44888: F, t44939: F, t45627: F, t45630: F, t45633: F, t45636: F, t45639: F, t45648: F, t45653: F, t45658: F, t45663: F, t45667: F, t5241: F, t5640: F, t590: F, t701: F, t8638: F) -> F {
    let t45672 = -F::new(0.63904876589867916126e-1) * t41244 - F::new(0.23005755572352449806e1) * t1998 * t1445 * t44939 * t701 - t45627 + t45630 - t45633 - t45636 + t45639 - F::new(0.21450293971110256002e1) * t8638 * t13682 - F::new(0.21450293971110256002e1) * t3025 * t1645 * t10668 + F::new(0.23005755572352449806e2) * t2197 * t13685 + t45648 - F::new(0.59584149919750711116e-1) * t43812 - F::new(0.59584149919750711116e-1) * t43817 + t45653 - F::new(0.50050685932590597338e1) * t10867 * t33436 + t45658 + F::new(0.30674340763136599742e1) * t5640 * t5241 * t44888 * t590 + t45663 - t45667 - F::new(0.51123901271894332902e1) * t1966 * t1890 * t44888 * t590;
    t45672
}
