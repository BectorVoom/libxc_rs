//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1304/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1304(t29922: f64, t659: f64, t2341: f64, t91: f64, t2342: f64, t2248: f64, t8138: f64, t29894: f64, t29896: f64, t29898: f64, t29901: f64, t29903: f64, t29904: f64, t29908: f64, t29912: f64, t29915: f64, t29919: f64, t64: f64, t8128: f64, t8137: f64) -> (f64, f64, f64, f64, f64) {
    let t29923 = t29922 * t659;
    let t29926 = t91 * t2341;
    let t29927 = t29926 * t2342;
    let t29930 = t8138 * t2248;
    let t29933 = -t29894 - 4.0_f64 / 3.0_f64 * t29896 - 10.0_f64 / 9.0_f64 * t29898 + 10.0_f64 / 9.0_f64 * t29901 - 3.0_f64 / 4.0_f64 * t29903 * t29904 - 5.0_f64 / 6.0_f64 * t8128 * t29908 + 5.0_f64 / 6.0_f64 * t8128 * t29912 + t8128 * t29915 / 4.0_f64 - 5.0_f64 / 9.0_f64 * t64 * t29919 + 25.0_f64 / 36.0_f64 * t8137 * t29923 - 5.0_f64 / 36.0_f64 * t8137 * t29927 - 5.0_f64 / 24.0_f64 * t8137 * t29930;
    (t29923, t29926, t29927, t29930, t29933)
}
