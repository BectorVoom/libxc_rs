//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 899/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk899<F: Float>(t1430: F, t444: F, t1440: F, t2499: F, t1429: F, t27: F, t2503: F, t82: F, t1419: F, t1437: F, t1441: F, t23: F, t2490: F, t2494: F, t434: F, t6655: F, t6658: F, t6659: F, t6662: F, t6665: F, t6668: F, t6676: F, t6679: F, t7: F, t974: F, t980: F) -> (F, F, F, F, F) {
    let t6680 = t1430 * t444;
    let t6683 = t2499 * t1440;
    let t6686 = t27 * t1429;
    let t6689 = t2503 * t82;
    let t6692 = F::new(440.0) / F::new(27.0) * t1419 * t974 - F::new(160.0) / F::new(27.0) * t434 * t2490 - F::new(80.0) / F::new(9.0) * t434 * t2494 - F::new(10.0) / F::new(27.0) * t7 * t6655 + F::new(20.0) / F::new(9.0) * t6658 * t6659 + F::new(10.0) / F::new(9.0) * t7 * t6662 + F::new(5.0) / F::new(3.0) * t7 * t6665 - F::new(5.0) * t7 * t6668 - F::new(80.0) / F::new(27.0) * t980 * t1437 - F::new(40.0) / F::new(9.0) * t980 * t1441 - F::new(10.0) / F::new(27.0) * t23 * t6676 - F::new(20.0) / F::new(9.0) * t6679 * t6680 + F::new(10.0) / F::new(9.0) * t23 * t6683 - F::new(5.0) / F::new(3.0) * t23 * t6686 + F::new(5.0) * t23 * t6689;
    (t6680, t6683, t6686, t6689, t6692)
}
