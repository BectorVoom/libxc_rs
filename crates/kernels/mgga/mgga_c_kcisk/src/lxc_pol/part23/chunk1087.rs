//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1087/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1087<F: Float>(t1210: F, t6557: F, t2297: F, t3716: F, t3697: F, t6560: F, t4437: F, t6544: F, t4463: F, t6540: F, t1536: F, t4455: F, t14800: F, t2292: F, t6541: F, t2285: F, t4435: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21839 = t6557 * t1210;
    let t21842 = t2297 * t3716;
    let t21845 = t6560 * t3697;
    let t21848 = t6544 * t4437;
    let t21851 = t6540 * t4463;
    let t21852 = t21851 * t1536;
    let t21855 = t6544 * t4455;
    let t21858 = t2292 * t14800;
    let t21859 = t21858 * t4437;
    let t21866 = t6541 * t1536;
    let t21869 = t2285 * t4435;
    (t21839, t21842, t21845, t21848, t21852, t21855, t21859, t21866, t21869)
}
