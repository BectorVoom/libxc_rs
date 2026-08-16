//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1139/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1139(t1390: f64, t22954: f64, t828: f64, t1388: f64, t13959: f64, t14013: f64, t14043: f64, t22156: f64, t22179: f64, t22183: f64, t22260: f64, t22264: f64, t22268: f64, t22285: f64, t22292: f64, t22914: f64, t9953: f64) -> (f64, f64) {
    let t22956 = t1390 * t828 * t22954;
    let t22962 = -0.17006693853500995666e-1_f64 * t13959 - 0.24009450146119052704e-1_f64 * t22156 - 0.5421477899694558815e-4_f64 * t14013 + 0.30011812682648815881e-2_f64 * t22179 + 0.76230004213927992337e-4_f64 * t22183 - 0.38115002106963996168e-4_f64 * t22260 - 0.17149607247227894789e-3_f64 * t22264 - 0.38115002106963996168e-4_f64 * t22268 - 0.21437009059034868486e-3_f64 * t1388 * t22914 - 0.21437009059034868486e-3_f64 * t1388 * t22956 - t9953 - 0.60023625365297631762e-2_f64 * t22285 + 0.30011812682648815881e-2_f64 * t22292 + 0.40656002247428262579e-3_f64 * t14043;
    (t22956, t22962)
}
