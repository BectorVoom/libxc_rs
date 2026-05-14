//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 593/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk593<F: Float>(t11724: F, t723: F, t1445: F, t3621: F, t590: F, t3626: F, t10814: F, t10824: F, t10829: F, t10835: F, t10838: F, t10841: F, t10844: F, t11721: F, t1966: F, t1991: F, t2103: F, t813: F) -> (F,) {
    let t11725 = t11724 * t723;
    let t11726 = t1445 * t11725;
    let t11730 = t3621 * t590;
    let t11733 = t3626 * t590;
    let t11742 = 0.71500979903700853338e0 * t2103 * t11721 - 0.92023022289409799224e1 * t813 * t11726 - 0.11916829983950142223e0 * t10814 + 0.1022478025437886658e1 * t1991 * t11730 - 0.25561950635947166451e1 * t1966 * t11733 - 0.76685851907841499353e0 * t10824 - 0.76685851907841499353e0 * t10829 + 0.59584149919750711116e-1 * t10835 - 0.1022478025437886658e1 * t10838 + 0.11916829983950142223e0 * t10841 + 0.1022478025437886658e1 * t10844;
    (t11742,)
}
