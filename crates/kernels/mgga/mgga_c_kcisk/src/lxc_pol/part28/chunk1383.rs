//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1383/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1383<F: Float>(t23957: F, t5054: F, t9679: F, t10461: F, t23025: F, t23029: F, t60514: F, t1333: F, t35152: F, t1790: F, t32935: F, t7261: F, t8820: F, t34118: F, t34122: F, t1871: F, t8665: F) -> (F, F, F, F, F, F, F) {
    let t121828 = t5054 * t9679 * t23957;
    let t121831 = t10461 * t9679 * t23025;
    let t121834 = t60514 * t9679 * t23029;
    let t121838 = t1333 * t35152;
    let t121851 = t7261 * t32935 * t8820 * t1790;
    let t121856 = t34122 * t34118;
    let t121860 = t8665 * t1871;
    (t121828, t121831, t121834, t121838, t121851, t121856, t121860)
}
