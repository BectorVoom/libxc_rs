//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 706/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk706(t10488: f64, t1224: f64, t4840: f64, t10585: f64, t10442: f64, t1697: f64, t10593: f64, t10450: f64, t10934: f64, t10937: f64, t10941: f64, t10944: f64, t10947: f64, t10951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10954 = t1224 * t4840 * t10488;
    let t10957 = t1224 * t4840 * t10585;
    let t10960 = t1224 * t1697 * t10442;
    let t10963 = t1224 * t1697 * t10593;
    let t10966 = t1224 * t1697 * t10450;
    let t10968 = -t10934 - 0.12361111111111111111e-1_f64 * t10937 + 0.61805555555555555556e-2_f64 * t10941 - 0.18541666666666666667e-1_f64 * t10944 + 0.92708333333333333334e-2_f64 * t10947 - 0.10300925925925925926e-1_f64 * t10951 + 0.37083333333333333333e-1_f64 * t10954 - 0.18541666666666666666e-1_f64 * t10957 - 0.55625000000000000001e-1_f64 * t10960 + 0.55625000000000000001e-1_f64 * t10963 - 0.92708333333333333333e-2_f64 * t10966;
    (t10954, t10957, t10960, t10963, t10966, t10968)
}
