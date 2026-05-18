//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 675/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk675<F: Float>(t10791: F, t397: F, t662: F, t656: F, t1782: F, t4893: F, t7233: F, t10487: F, t1849: F, t569: F, t1310: F, t10463: F) -> (F, F, F, F, F, F) {
    let t10793 = t397 * t10791 * t662;
    let t10795 = F::new(0.19989765240197019125e-1) * t656 * t10793;
    let t10798 = t4893 * t1782;
    let t10802 = t7233 * t1782;
    let t10812 = t662 * t10487;
    let t10831 = F::new(1.0) / t569 / t1849;
    let t10832 = t1310 * t10831;
    let t10833 = t662 * t10463;
    (t10795, t10798, t10802, t10812, t10832, t10833)
}
