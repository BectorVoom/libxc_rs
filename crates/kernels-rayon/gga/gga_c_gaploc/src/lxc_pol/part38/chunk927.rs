//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 927/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk927(t43914: f64, t43917: f64, t13625: f64, t825: f64, t826: f64, t13632: f64, t7416: f64, t10914: f64, t10915: f64, t45369: f64, t45316: f64, t7584: f64, t7585: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45736 = 0.59584149919750711116e-1_f64 * t43914;
    let t45737 = 0.59584149919750711116e-1_f64 * t43917;
    let t45741 = t825 * t826 * t13625;
    let t45743 = t7416 * t13632;
    let t45744 = 0.19171462976960374838e0_f64 * t45743;
    let t45747 = 0.21450293971110256001e2_f64 * t10914 * t10915 * t45369;
    let t45753 = 0.43710935587469654631e2_f64 * t7584 * t7585 * t45316;
    (t45736, t45737, t45741, t45744, t45747, t45753)
}
