//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 784/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk784<F: Float>(t1444: F, t160: F, t833: F, t2645: F, t4061: F, t1445: F, t2642: F, t1441: F, t532: F, t450: F, t4075: F, t743: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11951 = t160 * t1444;
    let t11952 = t11951 * t833;
    let t11954 = t4061 * t2645;
    let t11958 = t1445 * t2642;
    let t11960 = t1441 * t833;
    let t11962 = t532 * t2645;
    let t11966 = t160 * t450;
    let t11967 = F::new(0.71734315950379065738e-1) * t11966;
    let t11974 = t743 * t4075;
    (t11951, t11952, t11954, t11958, t11960, t11962, t11966, t11967, t11974)
}
