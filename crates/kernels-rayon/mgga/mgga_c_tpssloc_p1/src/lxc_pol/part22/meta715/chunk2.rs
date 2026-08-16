//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2323/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2323(t46447: f64, t5499: f64, t58972: f64, t12939: f64, t17635: f64, t4195: f64, t20217: f64, t707: f64, t751: f64, t1462: f64, t58976: f64, t39549: f64, t39563: f64, t39585: f64, t39590: f64, t40801: f64, t40803: f64, t67216: f64, t67217: f64, t67226: f64, t67228: f64, t67231: f64, t67244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67457 = 36.0_f64 * t46447 * t5499;
    let t67458 = 0.32530743900905219526e-1_f64 * t58972;
    let t67461 = 72.0_f64 * t12939 * t4195 * t17635;
    let t67463 = t707 * t751 * t20217;
    let t67464 = 4.0_f64 * t67463;
    let t67466 = 12.0_f64 * t58976 * t1462;
    let t67467 = t40801 - t40803 - t67216 + t67217 + t39549 + t39563 + t67226 + t67228 + t67231 + t67244 + t67457 + t67458 + t67461 + t67464 + t67466 - t39585 + t39590;
    (t67457, t67458, t67461, t67464, t67466, t67467)
}
