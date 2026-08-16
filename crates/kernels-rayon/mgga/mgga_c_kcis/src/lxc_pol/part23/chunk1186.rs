//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1186/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1186(t1494: f64, t94424: f64, t1598: f64, t37622: f64, t1014: f64, t27391: f64, t27345: f64, t7895: f64, t27348: f64, t18210: f64, t27341: f64, t2237: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94425 = t94424 * t1494;
    let t94440 = t37622 * t1598;
    let t94451 = t1014 * t27391;
    let t94465 = t7895 * t27345;
    let t94467 = t7895 * t27348;
    let t94469 = t18210 * t27341;
    let t94470 = t2237 * t94469;
    (t94425, t94440, t94451, t94465, t94467, t94469, t94470)
}
