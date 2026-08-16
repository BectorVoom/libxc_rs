//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 841/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk841(t30829: f64, t31769: f64, t544: f64, t913: f64, t1424: f64, t2875: f64, t9060: f64, t40202: f64, t3177: f64, t8272: f64, t9267: f64, t40208: f64) -> (f64, f64, f64, f64, f64) {
    let t41884 = t544 * t30829 * t913 * t31769;
    let t41885 = 0.3575048995185042667e0_f64 * t41884;
    let t41889 = 0.39722766613167140743e-1_f64 * t544 * t9060 * t2875 * t1424;
    let t41893 = 0.46011511144704899612e1_f64 * t40202;
    let t41903 = t9267 * t8272 * t3177;
    let t41904 = 0.19171462976960374838e1_f64 * t41903;
    let t41905 = 0.10352590007558602413e2_f64 * t40208;
    (t41885, t41889, t41893, t41904, t41905)
}
