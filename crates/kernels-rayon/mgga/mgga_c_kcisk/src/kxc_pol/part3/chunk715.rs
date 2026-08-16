//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 715/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk715(t11084: f64, t11099: f64, t1737: f64, t1746: f64, t10933: f64, t10937: f64, t10941: f64, t10944: f64, t10947: f64, t10951: f64, t10954: f64, t10957: f64, t10960: f64, t10963: f64, t10966: f64) -> (f64, f64) {
    let t11100 = t11084 + t11099;
    let t11102 = t1737 * t11100 * t1746;
    let t11105 = 0.55403703703703703703e-1_f64 * t10933;
    let t11116 = -t11105 - 0.23744444444444444444e-1_f64 * t10937 + 0.11872222222222222222e-1_f64 * t10941 - 0.35616666666666666666e-1_f64 * t10944 + 0.17808333333333333333e-1_f64 * t10947 - 0.19787037037037037037e-1_f64 * t10951 + 0.71233333333333333332e-1_f64 * t10954 - 0.35616666666666666666e-1_f64 * t10957 - 0.10685e0_f64 * t10960 + 0.10685e0_f64 * t10963 - 0.17808333333333333333e-1_f64 * t10966;
    (t11102, t11116)
}
