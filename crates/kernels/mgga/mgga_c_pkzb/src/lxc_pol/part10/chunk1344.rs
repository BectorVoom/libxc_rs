//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1344/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1344<F: Float>(t25849: F, t25855: F, t25857: F, t25859: F, t25861: F, t25863: F, t25865: F, t25867: F, t25869: F, t25872: F, t25876: F, t25878: F, t25880: F, t3698: F, t6065: F, t2993: F, t803: F) -> (F, F, F) {
    let t26805 = -t25849 - t25855 - t25857 + t25859 + t25861 - t25863 - t25865 - t25867 + t25869 - t25872 + t25876 + t25878 + t25880;
    let t26809 = t3698 * t6065;
    let t26813 = t2993 * t803;
    (t26805, t26809, t26813)
}
