//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1389/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1389(t34630: f64, t2890: f64, t6895: f64, t9267: f64, t20954: f64, t3407: f64, t10431: f64, t7014: f64, t10435: f64, t10525: f64, t2365: f64, t25723: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34631 = 0.19171462976960374838e1_f64 * t34630;
    let t34633 = t9267 * t2890 * t6895;
    let t34634 = 0.9585731488480187419e0_f64 * t34633;
    let t34635 = t20954 * t3407;
    let t34636 = 0.19171462976960374838e0_f64 * t34635;
    let t34637 = t7014 * t10431;
    let t34638 = 0.38342925953920749676e0_f64 * t34637;
    let t34639 = t7014 * t10435;
    let t34640 = 0.85206502119823888168e-1_f64 * t34639;
    let t34642 = t10525 * t2365 * t25723;
    (t34631, t34634, t34636, t34638, t34640, t34642)
}
