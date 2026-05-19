//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 693/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk693<F: Float>(t10791: F, t397: F, t662: F, t656: F, t1782: F, t4893: F, t5016: F, t5013: F, t7233: F, t1785: F, t4640: F, t4998: F, t5021: F) -> (F, F, F, F) {
    let t10793 = t397 * t10791 * t662;
    let t10795 = F::cast_from(0.19989765240197019125e-1_f64) * t656 * t10793;
    let t10798 = t4893 * t1782;
    let t10799 = t10798 * t5016;
    let t10800 = t5013 * t10799;
    let t10802 = t7233 * t1782;
    let t10803 = t4640 * t1785;
    let t10804 = t10802 * t10803;
    let t10809 = t4998 * t5021;
    (t10795, t10800, t10804, t10809)
}
