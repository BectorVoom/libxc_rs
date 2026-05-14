//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1332/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1332<F: Float>(t2922: F, t774: F, t9278: F, t7664: F, t9283: F, t5925: F, t9613: F, t3652: F, t5939: F, t757: F, t3650: F, t5728: F, t18018: F, t2009: F, t2096: F, t2104: F, t2105: F, t21930: F, t21933: F, t21935: F, t25155: F, t2900: F, t302: F, t3542: F, t3641: F, t3653: F, t3679: F, t5693: F, t5945: F, t761: F, t7648: F, t7658: F, t7666: F, t9542: F, t9562: F) -> (F, F) {
    let t26510 = t2922 * t774 * t9278;
    let t26513 = t7664 * t774 * t9283;
    let t26527 = t5925 * t9613;
    let t26535 = t757 * t5939 * t3652;
    let t26537 = t3650 * t5728;
    let t26542 = -0.21437009059034868486e-3 * t2922 * t302 * t9562 * t7658 + 0.12862205435420921092e-2 * t2104 * t5693 * t3542 * t2009 * t761 - 0.57165357490759649296e-3 * t26510 + 0.28582678745379824648e-3 * t26513 + 0.42874018118069736972e-3 * t2922 * t2105 * t3679 * t7648 - 0.42874018118069736972e-3 * t2922 * t302 * t2900 * t25155 + 0.72409452821628889107e-2 * t5945 * t3653 + 0.96545937095505185476e-2 * t21930 + 0.1270341277572436651e-3 * t21933 - 0.30488190661738479624e-2 * t26527 - 0.22866142996303859718e-2 * t2096 * t9542 + 0.14481890564325777821e-1 * t18018 * t3641 + 0.10162730220579493208e-2 * t21935 - 0.47637797908966374413e-4 * t26535 + 0.21437009059034868486e-3 * t7664 * t302 * t26537 * t7666;
    (t26537, t26542)
}
