//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1389/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1389<F: Float>(t34630: F, t2890: F, t6895: F, t9267: F, t20954: F, t3407: F, t10431: F, t7014: F, t10435: F, t10525: F, t2365: F, t25723: F) -> (F, F, F, F, F, F) {
    let t34631 = F::cast_from(0.19171462976960374838e1_f64) * t34630;
    let t34633 = t9267 * t2890 * t6895;
    let t34634 = F::cast_from(0.9585731488480187419e0_f64) * t34633;
    let t34635 = t20954 * t3407;
    let t34636 = F::cast_from(0.19171462976960374838e0_f64) * t34635;
    let t34637 = t7014 * t10431;
    let t34638 = F::cast_from(0.38342925953920749676e0_f64) * t34637;
    let t34639 = t7014 * t10435;
    let t34640 = F::cast_from(0.85206502119823888168e-1_f64) * t34639;
    let t34642 = t10525 * t2365 * t25723;
    (t34631, t34634, t34636, t34638, t34640, t34642)
}
