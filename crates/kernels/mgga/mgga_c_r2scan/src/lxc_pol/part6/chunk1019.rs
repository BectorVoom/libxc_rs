//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1019/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1019<F: Float>(t2062: F, t7872: F, t2823: F, t5998: F, t6027: F, t897: F, t6029: F, t4827: F, t4839: F, t4996: F, t5000: F, t5004: F, t5008: F, t7015: F, t7870: F, t4842: F, t4845: F, t5020: F, t6010: F, t6012: F, t7020: F, t7021: F, t7025: F, t7031: F, t7033: F, t7036: F) -> (F, F, F) {
    let t7874 = 0.1350520664e0 * t7872 * t2062;
    let t7876 = 0.1350520664e0 * t2823 * t5998;
    let t7877 = t6027 * t897;
    let t7878 = t7877 * t6029;
    let t7880 = -0.675260332e-1 * t7870 - t7874 - t7876 + 0.1350520664e0 * t7878 - t4996 + t5000 + t5004 + t5008 + t7015 + t4827 - t4839;
    let t7884 = t7020 - t7021 + t5020 + t6010 - 0.1143056e0 * t6012 - t4842 - t7025 - t7031 - t7033 + t7036 + t4845;
    (t7877, t7880, t7884)
}
