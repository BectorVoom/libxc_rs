//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 664/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk664<F: Float>(t10933: F, t10937: F, t10941: F, t10944: F, t10947: F, t10951: F, t10954: F, t10957: F, t10960: F, t10963: F, t10966: F, t1701: F, t4857: F, t10926: F, t1725: F, t1726: F, t4903: F) -> (F, F, F, F) {
    let t11105 = 0.55403703703703703703e-1 * t10933;
    let t11116 = -t11105 - 0.23744444444444444444e-1 * t10937 + 0.11872222222222222222e-1 * t10941 - 0.35616666666666666666e-1 * t10944 + 0.17808333333333333333e-1 * t10947 - 0.19787037037037037037e-1 * t10951 + 0.71233333333333333332e-1 * t10954 - 0.35616666666666666666e-1 * t10957 - 0.10685e0 * t10960 + 0.10685e0 * t10963 - 0.17808333333333333333e-1 * t10966;
    let t11119 = t1701 * t4857;
    let t11122 = t10926 * t1725;
    let t11125 = t1726 * t4903;
    (t11116, t11119, t11122, t11125)
}
