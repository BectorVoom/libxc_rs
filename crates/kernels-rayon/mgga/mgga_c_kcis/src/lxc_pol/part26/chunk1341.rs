//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1341/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1341(t15808: f64, t2066: f64, t28589: f64, t5919: f64, t22361: f64, t27520: f64, t12266: f64, t7318: f64, t102937: f64, t102939: f64, t102942: f64, t102944: f64, t102946: f64, t102948: f64, t102950: f64) -> (f64, f64, f64, f64, f64) {
    let t102952 = t15808 * t2066;
    let t102954 = t28589 * t5919;
    let t102956 = t27520 * t22361;
    let t102958 = t12266 * t7318;
    let t102960 = -0.125e0_f64 * t102937 + 0.91666666666666666667e0_f64 * t102939 + 0.61111111111111111111e0_f64 * t102942 + 0.4046875e-1_f64 * t102944 - 0.5e0_f64 * t102946 - 0.21583333333333333334e0_f64 * t102948 - 0.625e-1_f64 * t102950 - 0.53958333333333333334e-1_f64 * t102952 - 0.125e0_f64 * t102954 + 0.1875e0_f64 * t102956 - 0.4046875e-1_f64 * t102958;
    (t102952, t102954, t102956, t102958, t102960)
}
