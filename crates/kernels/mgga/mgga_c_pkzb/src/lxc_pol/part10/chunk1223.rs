//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1223/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1223<F: Float>(t411: F, t6546: F, t8480: F, t914: F, t10390: F, t6755: F, t6756: F, t7221: F, t7915: F, t8593: F, t8709: F, t8715: F, t9128: F, t9129: F, t9744: F, t1413: F, t1444: F, t1449: F, t1450: F, t1466: F, t16036: F, t19467: F, t19470: F, t2507: F, t2510: F, t2513: F, t2528: F, t3311: F, t3337: F, t3340: F, t3356: F, t459: F, t4772: F, t4828: F, t6634: F, t6747: F, t8604: F, t8607: F, t8610: F, t8615: F, t8661: F, t8664: F, t8673: F, t8705: F, t987: F, t995: F) -> (F, F, F, F) {
    let t23398 = t411 * t6546;
    let t23498 = t914 * t8480;
    let t23617 = -0.478125e-1 * t8593 + 4.0 * t6756 - 0.478125e-1 * t7915 + 2.0 * t9128 + 2.0 * t6755 + 2.0 * t8709 + 2.0 * t8715 + 0.19125e0 * t7221 - 0.478125e-1 * t10390 - 0.478125e-1 * t9744 + 0.95625e-1 * t9129;
    let t23683 = -0.33125e-1 * t1413 * t3356 * t1444 + 0.99375e-1 * t1449 * t8705 * t459 + 0.3975e0 * t4772 * t8610 * t459 + 0.19875e0 * t4772 * t3311 * t1466 - 0.795e0 * t4828 * t2513 * t2528 + 0.3975e0 * t19467 * t8604 + 0.3975e0 * t4772 * t8607 * t459 + 0.19875e0 * t4772 * t8673 * t459 - 0.795e0 * t16036 * t8664 * t459 + 0.3975e0 * t4772 * t2510 * t2528 - 0.795e0 * t16036 * t3311 * t1450 + 0.99375e-1 * t4772 * t3340 * t1444 - 0.19875e0 * t4828 * t3340 * t1466 + 0.99375e-1 * t1449 * t995 * t6747 - 0.1325e0 * t1413 * t2507 * t2528 - 0.6625e-1 * t1413 * t987 * t6747 - 0.6625e-1 * t6634 * t8615 - 0.6625e-1 * t1413 * t8661 * t459 - 0.33125e-1 * t1413 * t3337 * t1466 - 0.6625e-1 * t19470 * t3311;
    (t23398, t23498, t23617, t23683)
}
