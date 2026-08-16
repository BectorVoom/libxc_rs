//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 924/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk924(t40202: f64, t2375: f64, t34558: f64, t2478: f64, t3358: f64, t6576: f64, t3177: f64, t8272: f64, t9267: f64, t40208: f64, t12953: f64, t4781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41893 = 0.46011511144704899612e1_f64 * t40202;
    let t41897 = t34558 * t2375;
    let t41900 = t6576 * t3358 * t2478;
    let t41903 = t9267 * t8272 * t3177;
    let t41904 = 0.19171462976960374838e1_f64 * t41903;
    let t41905 = 0.10352590007558602413e2_f64 * t40208;
    let t41906 = t4781 * t12953;
    (t41893, t41897, t41900, t41904, t41905, t41906)
}
