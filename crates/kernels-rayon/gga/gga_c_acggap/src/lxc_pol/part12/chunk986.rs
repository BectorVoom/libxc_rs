//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 986/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk986(t33175: f64, t7963: f64, t7965: f64, t2132: f64, t2138: f64, t3101: f64, t633: f64, t4210: f64, t7942: f64, t3645: f64, t635: f64, t8114: f64, t880: f64) -> (f64, f64, f64, f64, f64) {
    let t33180 = t7963 * t33175 * t7965;
    let t33185 = 0.8673628188205199462e0_f64 * t2138 * t2132 * t633 * t3101;
    let t33198 = t7942 * t33175 * t4210;
    let t33201 = 0.65854491829355115987e0_f64 * t3645 * t635;
    let t33208 = t8114 * t880;
    (t33180, t33185, t33198, t33201, t33208)
}
