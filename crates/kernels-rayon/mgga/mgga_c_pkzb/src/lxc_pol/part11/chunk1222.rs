//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1222/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1222(t10944: f64, t2099: f64, t5933: f64, t1107: f64, t21267: f64, t26211: f64, t721: f64, t2866: f64, t9242: f64, t20683: f64, t9229: f64, t1899: f64, t2782: f64, t3525: f64) -> (f64, f64, f64, f64, f64) {
    let t30164 = t5933 * t2099 * t10944;
    let t30193 = 0.30762056574649219973e4_f64 * t21267 * t26211 * t1107 * t721;
    let t30195 = 0.35089341735807877242e1_f64 * t9242 * t2866;
    let t30197 = 0.2894756309764656312e3_f64 * t20683 * t9229;
    let t30200 = 18.0_f64 * t1899 * t3525 * t2782;
    (t30164, t30193, t30195, t30197, t30200)
}
