//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1309/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1309<F: Float>(t1901: F, t9389: F, t1899: F, t683: F, t5771: F, t9232: F, t17536: F, t9236: F, t25855: F, t25857: F, t25859: F, t25861: F, t25863: F, t25865: F, t25867: F, t25869: F, t25872: F) -> (F, F, F, F) {
    let t25873 = t9389 * t1901;
    let t25876 = 0.32163958997385070134e2 * t1899 * t25873 * t683;
    let t25878 = 0.64327917994770140268e2 * t5771 * t9232;
    let t25880 = 0.1034520258385468006e4 * t17536 * t9236;
    let t25881 = -t25855 - t25857 + t25859 + t25861 - t25863 - t25865 - t25867 + t25869 - t25872 + t25876 + t25878 + t25880;
    (t25876, t25878, t25880, t25881)
}
