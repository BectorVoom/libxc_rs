//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1276/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1276(t28343: f64, t94246: f64, t7908: f64, t27416: f64, t27447: f64, t27459: f64, t28353: f64, t28420: f64, t28544: f64, t37602: f64, t491: f64, t8159: f64, t98174: f64, t98823: f64, t98825: f64, t98830: f64, t98835: f64, t98845: f64, t990: f64) -> (f64, f64) {
    let t98847 = t94246 * t28343;
    let t98849 = 0.46336805555555555556e-3_f64 * t7908 * t98847;
    let t98850 = -t98823 + t98825 - 0.24734586805555555555e-3_f64 * t28544 * t27416 + 0.69505208333333333333e-3_f64 * t27447 * t8159 - 0.3684876543209876543e-3_f64 * t98830 + 0.41703125000000000001e-2_f64 * t7908 * t98174 + 0.88437037037037037034e-2_f64 * t98835 - 0.37134344353515625e-4_f64 * t37602 * t491 * t990 * t28353 + 0.92673611111111111112e-3_f64 * t27459 * t28420 - 0.92673611111111111113e-3_f64 * t98845 - t98849;
    (t98847, t98850)
}
