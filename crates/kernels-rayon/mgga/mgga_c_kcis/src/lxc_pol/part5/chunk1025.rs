//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1025/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1025(t1697: f64, t2835: f64, t1141: f64, t5034: f64, t1778: f64, t3329: f64, t13105: f64, t381: f64, t1795: f64, t3225: f64, t3436: f64, t5025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14654 = t1697 * t2835;
    let t14665 = t5034 * t1141;
    let t14668 = t1778 * t3329;
    let t14721 = t13105 * t381;
    let t14781 = t1795 * t3225;
    let t14785 = t5025 * t3436;
    (t14654, t14665, t14668, t14721, t14781, t14785)
}
