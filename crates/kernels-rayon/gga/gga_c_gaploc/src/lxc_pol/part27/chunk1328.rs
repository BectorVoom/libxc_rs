//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1328/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1328(t34633: f64, t20954: f64, t3407: f64, t10431: f64, t7014: f64, t10435: f64, t10525: f64, t2365: f64, t25723: f64, t10514: f64, t21370: f64, t10531: f64, t10534: f64, t1406: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34634 = 0.9585731488480187419e0_f64 * t34633;
    let t34635 = t20954 * t3407;
    let t34636 = 0.19171462976960374838e0_f64 * t34635;
    let t34637 = t7014 * t10431;
    let t34638 = 0.38342925953920749676e0_f64 * t34637;
    let t34639 = t7014 * t10435;
    let t34640 = 0.85206502119823888168e-1_f64 * t34639;
    let t34642 = t10525 * t2365 * t25723;
    let t34643 = 0.89376224879626066674e-1_f64 * t34642;
    let t34645 = 0.12423108009070322895e3_f64 * t21370 * t10514;
    let t34648 = 0.55213813373645879534e2_f64 * t1406 * t10531 * t10534;
    (t34634, t34636, t34638, t34640, t34643, t34645, t34648)
}
