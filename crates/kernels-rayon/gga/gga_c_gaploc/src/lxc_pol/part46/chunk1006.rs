//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1006/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1006(t13142: f64, t7416: f64, t2365: f64, t32215: f64, t6111: f64, t13019: f64, t4614: f64, t833: f64, t11001: f64, t2714: f64, t2718: f64, t33725: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44009 = t7416 * t13142;
    let t44010 = 0.15976219147466979032e-1_f64 * t44009;
    let t44012 = t6111 * t2365 * t32215;
    let t44018 = t833 * t4614 * t13019;
    let t44020 = t2714 * t11001;
    let t44022 = t2718 * t11001;
    let t44024 = t955 * t33725;
    (t44010, t44012, t44018, t44020, t44022, t44024)
}
