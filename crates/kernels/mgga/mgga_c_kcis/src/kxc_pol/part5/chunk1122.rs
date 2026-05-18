//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1122/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1122<F: Float>(t18929: F, t26: F, t18672: F, t2970: F, t6383: F, t659: F, t6386: F, t13710: F, t13945: F, t13949: F, t18924: F, t18927: F, t9726: F, t9729: F) -> (F, F, F, F, F) {
    let t18930 = t26 * t18929;
    let t18932 = t2970 * t18672;
    let t18933 = t26 * t18932;
    let t18935 = t659 * t6383;
    let t18937 = t659 * t6386;
    let t18942 = -F::new(0.49293999999999999999e0) * t18924 + F::new(0.65725333333333333332e0) * t18927 + F::new(0.16431333333333333333e0) * t18930 - F::new(0.27385555555555555556e-1) * t18933 - t9726 - t9729 - F::new(0.10954222222222222222e0) * t18935 + F::new(0.54771111111111111111e-1) * t18937 - F::new(0.18257037037037037037e0) * t13945 - F::new(0.26574814814814814815e0) * t13710 + F::new(0.21908444444444444444e0) * t13949;
    (t18930, t18933, t18935, t18937, t18942)
}
