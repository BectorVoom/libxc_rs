//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1126/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1126(t4718: f64, t4722: f64, t2986: f64, t6365: f64, t9770: f64, t949: f64, t9768: f64, t3031: f64, t6423: f64, t4764: f64, t13864: f64, t4690: f64) -> (f64, f64, f64, f64) {
    let t18981 = t4722 * t4718;
    let t18983 = 0.32163648644302209644e2_f64 * t2986 * t18981;
    let t18984 = t6365 * t9770;
    let t18985 = t18984 * t949;
    let t18987 = 0.51725014705706168417e3_f64 * t9768 * t18985;
    let t18988 = t3031 * t6423;
    let t18989 = t18988 * t4764;
    let t18993 = 4.0_f64 * t13864 * t4690;
    (t18983, t18987, t18989, t18993)
}
