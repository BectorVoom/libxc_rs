//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1272/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1272(t33585: f64, t2028: f64, t2536: f64, t787: f64, t8632: f64, t10007: f64, t10627: f64, t15482: f64, t22628: f64, t10930: f64, t10931: f64, t32893: f64) -> (f64, f64, f64, f64) {
    let t33586 = 0.19171462976960374838e1_f64 * t33585;
    let t33590 = 0.79445533226334281486e-1_f64 * t787 * t2536 * t8632 * t2028;
    let t33601 = t10007 * t10627;
    let t33604 = 0.22721733898619703511e0_f64 * t22628 * t15482 * t33601;
    let t33607 = 0.27606906686822939767e2_f64 * t10930 * t10931 * t32893;
    (t33586, t33590, t33604, t33607)
}
