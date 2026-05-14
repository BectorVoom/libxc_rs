//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 642/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk642<F: Float>(t13630: F, t883: F, t2685: F, t2684: F, t13625: F, t7428: F, t7427: F, t969: F, t825: F, t13506: F, t1445: F, t2087: F, t11622: F, t935: F, t813: F, t13555: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13631 = t13630 * t883;
    let t13632 = t2685 * t13631;
    let t13633 = t2684 * t13632;
    let t13634 = 0.19171462976960374838e0 * t13633;
    let t13635 = t7428 * t13625;
    let t13636 = t7427 * t13635;
    let t13638 = t2685 * t13625;
    let t13639 = t2684 * t13638;
    let t13641 = t969 * t13631;
    let t13642 = t825 * t13641;
    let t13643 = 0.19171462976960374838e0 * t13642;
    let t13644 = t1445 * t13506;
    let t13646 = 0.62115540045351614476e2 * t2087 * t13644;
    let t13647 = t11622 * t935;
    let t13648 = t1445 * t13647;
    let t13650 = 0.46011511144704899612e1 * t813 * t13648;
    let t13651 = t1445 * t13555;
    (t13631, t13632, t13634, t13635, t13636, t13638, t13639, t13641, t13643, t13644, t13646, t13647, t13648, t13650, t13651)
}
