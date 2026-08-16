//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 555/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk555(t3498: f64, t831: f64, t741: f64, t95: f64, t568: f64, t952: f64, t205: f64, t198: f64, t733: f64, t142: f64, t3163: f64, t957: f64) -> (f64, f64, f64, f64, f64) {
    let t3500 = 24.847010690177285_f64 * t831 * t3498;
    let t3501 = t741 * t95;
    let t3506 = t568 * t952;
    let t3507 = t205 * t3506;
    let t3509 = t198 * t733;
    let t3510 = t3509 * t142;
    let t3512 = 4.855032390388656_f64 * t3510 * t3163;
    let t3514 = 3.2366882602591036_f64 * t957 * t3498;
    (t3500, t3501, t3507, t3512, t3514)
}
