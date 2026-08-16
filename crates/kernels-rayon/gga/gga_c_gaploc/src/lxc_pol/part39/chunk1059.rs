//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1059/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1059(t3277: f64, t44090: f64, t13009: f64, t5782: f64, t1457: f64, t43240: f64, t6060: f64, t13158: f64, t15766: f64, t41430: f64, t41435: f64, t41445: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44092 = 0.16683561977530199113e1_f64 * t3277 * t44090;
    let t44093 = t5782 * t13009;
    let t44097 = 0.21450293971110256001e1_f64 * t6060 * t1457 * t43240;
    let t44099 = 0.21450293971110256001e1_f64 * t15766 * t13158;
    let t44110 = 0.19171462976960374838e1_f64 * t41430;
    let t44111 = 0.42603251059911944084e0_f64 * t41435;
    let t44112 = 0.29792074959875355558e-1_f64 * t41445;
    (t44092, t44093, t44097, t44099, t44110, t44111, t44112)
}
