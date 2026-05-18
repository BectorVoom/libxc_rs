//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1319/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1319<F: Float>(t4142: F, t6909: F, t11882: F, t12231: F, t15983: F, t15987: F, t21811: F, t21816: F, t21819: F, t21822: F, t21825: F, t21828: F, t3961: F) -> (F, F) {
    let t21834 = t4142 * t6909;
    let t21837 = -F::new(0.66327777777777777776e-2) * t21811 + F::new(0.55273148148148148147e-2) * t21816 - F::new(0.55273148148148148147e-3) * t21819 + F::new(0.49745833333333333332e-2) * t21822 + F::new(0.13265555555555555555e-1) * t21825 - F::new(0.2671335375e-1) * t3961 * t21828 - F::new(0.178244852896875e-2) * t12231 * t21828 - F::new(0.36848765432098765431e-3) * t11882 - F::new(0.58958024691358024689e-2) * t21834 + F::new(0.29479012345679012345e-2) * t15983 - t15987;
    (t21834, t21837)
}
