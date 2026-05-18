//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 706/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk706<F: Float>(t10488: F, t1224: F, t4840: F, t10585: F, t10442: F, t1697: F, t10593: F, t10450: F, t10934: F, t10937: F, t10941: F, t10944: F, t10947: F, t10951: F) -> (F, F, F, F, F, F) {
    let t10954 = t1224 * t4840 * t10488;
    let t10957 = t1224 * t4840 * t10585;
    let t10960 = t1224 * t1697 * t10442;
    let t10963 = t1224 * t1697 * t10593;
    let t10966 = t1224 * t1697 * t10450;
    let t10968 = -t10934 - F::new(0.12361111111111111111e-1) * t10937 + F::new(0.61805555555555555556e-2) * t10941 - F::new(0.18541666666666666667e-1) * t10944 + F::new(0.92708333333333333334e-2) * t10947 - F::new(0.10300925925925925926e-1) * t10951 + F::new(0.37083333333333333333e-1) * t10954 - F::new(0.18541666666666666666e-1) * t10957 - F::new(0.55625000000000000001e-1) * t10960 + F::new(0.55625000000000000001e-1) * t10963 - F::new(0.92708333333333333333e-2) * t10966;
    (t10954, t10957, t10960, t10963, t10966, t10968)
}
