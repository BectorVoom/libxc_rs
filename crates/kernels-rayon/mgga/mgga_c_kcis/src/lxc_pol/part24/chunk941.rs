//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 941/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk941(t1175: f64, t6704: f64, t375: f64, t3393: f64, t6669: f64, t18570: f64, t5142: f64, t18653: f64, t5134: f64, t15022: f64, t18648: f64, t18657: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19969 = t1175 * t6704;
    let t19970 = t375 * t19969;
    let t19972 = t3393 * t6669;
    let t19974 = t5142 * t18570;
    let t19977 = t5134 * t18653;
    let t19980 = t15022 * t18648;
    let t19983 = t5134 * t18657;
    (t19970, t19972, t19974, t19977, t19980, t19983)
}
