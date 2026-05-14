//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 675/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk675<F: Float>(t1994: F, t2030: F, t2648: F, t4812: F, t4814: F, t5075: F, t5348: F, t5445: F, t6954: F, t6959: F, t6963: F, t6968: F, t6971: F, t6976: F, t6979: F, t6983: F, t6988: F, t6990: F, t6992: F, t7072: F, t7553: F, t7645: F, t7648: F) -> (F,) {
    let t7653 = -0.17411041666666666666e-2 * t6954 - 0.11607361111111111111e-2 * t4812 + 0.77382407407407407407e-3 * t4814 + 0.77382407407407407407e-3 * t6959 - 0.193e0 * t5348 * t2648 + 0.46429444444444444443e-2 * t6963 - 0.30952962962962962962e-2 * t6968 + 0.11607361111111111111e-2 * t6971 - 0.17411041666666666666e-2 * t6976 + 0.11607361111111111111e-2 * t6979 + 0.77382407407407407407e-3 * t6983 - 0.11607361111111111111e-2 * t6988 - 0.30952962962962962962e-2 * t6990 + 0.11607361111111111111e-2 * t6992 + 0.77382407407407407407e-3 * t5075 - 0.17411041666666666666e-2 * t7072 + 0.193e0 * t1994 * t7553 - 0.193e0 * t1994 * t7645 - 0.193e0 * t7648 * t2030 + 0.74498e-1 * t5445 * t7553;
    (t7653,)
}
