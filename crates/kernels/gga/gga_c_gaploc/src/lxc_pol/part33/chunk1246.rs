//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1246/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1246<F: Float>(t3720: F, t701: F, t12214: F, t13846: F, t1890: F, t1986: F, t2103: F, t22767: F, t28022: F, t32748: F, t32753: F, t32756: F, t32759: F, t32761: F, t32764: F, t32766: F, t32769: F, t32771: F, t32774: F, t38907: F, t4673: F, t5241: F, t5640: F, t5840: F, t590: F, t739: F) -> (F, F) {
    let t38912 = t3720 * t701;
    let t38924 = -t32748 + 0.30674340763136599742e1 * t5640 * t5241 * t38907 * t590 + 0.1022478025437886658e1 * t5840 * t1890 * t38912 * t590 - 0.1022478025437886658e1 * t1986 * t739 * t13846 * t22767 - t28022 + 0.95334639871601137784e0 * t2103 * t4673 * t12214 + t32753 - t32756 - t32759 + t32761 - t32764 - t32766 + t32769 + t32771 + t32774;
    (t38912, t38924)
}
