//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1062/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1062(t1748: f64, t7731: f64, t303: f64, t1014: f64, t8051: f64, t26695: f64, t4547: f64, t4939: f64, t27773: f64, t922: f64, t2894: f64, t4781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27940 = t1748 * t7731;
    let t27941 = t303 * t27940;
    let t27947 = t1014 * t8051;
    let t27949 = t26695 * t4547;
    let t27950 = t4939 * t27949;
    let t27953 = t27773 * t922;
    let t27954 = t2894 * t27953;
    let t27957 = t4781 * t922;
    (t27940, t27941, t27947, t27949, t27950, t27953, t27954, t27957)
}
