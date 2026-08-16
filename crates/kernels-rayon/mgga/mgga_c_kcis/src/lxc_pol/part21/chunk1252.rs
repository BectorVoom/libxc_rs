//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1252/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1252(t2651: f64, t7671: f64, t26654: f64, t838: f64, t26633: f64, t26652: f64, t26420: f64, t27731: f64, t27733: f64, t27735: f64, t27737: f64, t27739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93817 = t2651 * t7671;
    let t93826 = t838 * t26654;
    let t93848 = 3.0_f64 * t26633;
    let t93849 = 3.0_f64 * t26652;
    let t93852 = 12.0_f64 * t26420;
    let t95270 = t27731 / 8.0_f64;
    let t95271 = 2.0_f64 * t27733;
    let t95272 = t27735 / 8.0_f64;
    let t95273 = t27737 / 8.0_f64;
    let t95274 = t27739 / 8.0_f64;
    (t93817, t93826, t93848, t93849, t93852, t95270, t95271, t95272, t95273, t95274)
}
