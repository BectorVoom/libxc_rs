//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1291/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1291<F: Float>(t17381: F, t730: F, t9531: F, t2875: F, t7560: F, t1999: F, t9242: F, t17637: F, t3591: F, t7227: F, t2848: F, t1954: F, t722: F, t1987: F, t9356: F, t1980: F) -> (F, F, F, F, F, F, F, F) {
    let t25588 = 0.10254018858216406658e4 * t730 * t9531 * t17381;
    let t25590 = 0.69263436422725855034e2 * t7560 * t2875;
    let t25592 = 0.17315859105681463759e2 * t9242 * t1999;
    let t25596 = 0.12304822629859687989e5 * t730 * t17637 * t3591 * t7227;
    let t25597 = t2848 * t2848;
    let t25601 = 0.23392894490538584828e1 * t730 * t1954 * t25597 * t722;
    let t25603 = 0.23392894490538584828e1 * t1987 * t9356;
    let t25606 = 0.6233709278045326953e3 * t730 * t9531 * t1980;
    (t25588, t25590, t25592, t25596, t25597, t25601, t25603, t25606)
}
