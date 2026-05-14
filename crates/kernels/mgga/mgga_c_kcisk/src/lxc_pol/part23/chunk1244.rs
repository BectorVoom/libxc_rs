//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1244/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1244<F: Float>(t1390: F, t1597: F, t5626: F, t32464: F, t3951: F, t539: F, t1310: F, t3532: F, t9511: F, t9859: F, t2740: F, t32439: F, t33513: F, t33530: F, t33533: F, t33535: F, t33542: F, t33545: F, t33906: F, t33911: F, t9536: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33914 = t1597 * t1390;
    let t33915 = t33914 * t5626;
    let t33916 = t32464 * t33915;
    let t33921 = t3951 * t539;
    let t33922 = t1310 * t33921;
    let t33923 = t1597 * t3532;
    let t33924 = t33923 * t5626;
    let t33925 = t33922 * t33924;
    let t33928 = t9511 * t9859;
    let t33933 = 0.46429444444444444443e-2 * t33513 + 0.77382407407407407407e-3 * t33530 - 0.30952962962962962963e-2 * t33533 - 0.11607361111111111111e-2 * t33535 + 0.17361111111111111111e-2 * t9536 * t33906 + 0.17361111111111111111e-2 * t9536 * t33911 + 0.34722222222222222222e-2 * t9536 * t33916 + 0.67013888888888888888e-3 * t32439 * t33911 - 0.23148148148148148148e-2 * t9536 * t33925 - 0.52083333333333333333e-2 * t33928 * t2740 + 0.77382407407407407407e-3 * t33542 - 0.23214722222222222222e-2 * t33545;
    (t33914, t33915, t33916, t33921, t33922, t33923, t33924, t33925, t33928, t33933)
}
