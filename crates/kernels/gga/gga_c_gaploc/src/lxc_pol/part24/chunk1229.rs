//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1229/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1229<F: Float>(t6798: F, t8411: F, t10351: F, t20496: F, t20671: F, t27003: F, t31041: F, t10597: F, t31051: F, t2482: F, t8272: F, t9267: F, t2890: F, t6895: F, t20954: F, t3407: F) -> (F, F, F, F, F, F, F) {
    let t34621 = 0.14300195980740170668e1 * t8411 * t6798;
    let t34623 = 0.13803453343411469884e2 * t20496 * t10351;
    let t34625 = t31041 * t20671 * t27003;
    let t34626 = 0.17041300423964777634e0 * t34625;
    let t34627 = t31051 * t10597;
    let t34628 = 0.19171462976960374838e1 * t34627;
    let t34630 = t9267 * t8272 * t2482;
    let t34631 = 0.19171462976960374838e1 * t34630;
    let t34633 = t9267 * t2890 * t6895;
    let t34634 = 0.9585731488480187419e0 * t34633;
    let t34635 = t20954 * t3407;
    (t34621, t34623, t34626, t34628, t34631, t34634, t34635)
}
