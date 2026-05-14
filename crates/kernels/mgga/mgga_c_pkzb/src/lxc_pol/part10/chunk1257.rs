//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1257/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1257<F: Float>(t133: F, t158: F, t160: F, t162: F, t1634: F, t1692: F, t1750: F, t1773: F, t20592: F, t23992: F, t24524: F, t24525: F, t24541: F, t24614: F, t24632: F, t24646: F, t24659: F, t24676: F, t2625: F, t2631: F, t2633: F, t2636: F, t3396: F, t3431: F, t568: F, t596: F, t614: F, t7065: F, t7070: F, t7075: F, t7078: F, t8817: F, t8865: F, t8876: F, t8881: F, t8882: F) -> (F,) {
    let t24714 = -(t24524 + t24525 + t24541 + t24614 + t24632 + t24646 + t24659 + t24676) * t158 * t162 - 24.0 * t8865 * t7078 + 60.0 * t2631 * t1773 * t3396 * t1634 + 12.0 * t2625 * t2636 - 48.0 * t2625 * t133 * t2633 - 48.0 * t8865 * t7075 + 3.0 * t3431 * t1750 + 3.0 * t160 * t596 * t23992 - 48.0 * t7065 * t8876 - 24.0 * t2631 * t614 * t8817 * t568 - 12.0 * t2631 * t8881 * t1692 - 24.0 * t7065 * t8882 + 240.0 * t2631 * t7070 * t20592;
    (t24714,)
}
