//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 789/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk789<F: Float>(t2082: F, t775: F, t2065: F, t771: F, t1485: F, t178: F, t301: F, t299: F, t1843: F, t655: F, t779: F, t2888: F) -> (F, F, F, F, F, F) {
    let t5607 = t2082 * t775;
    let t5609 = t771 * t2065;
    let t5612 = t178 * t1485 * t301;
    let t5614 = F::new(0.63517063878621832551e-4) * t299 * t5612;
    let t5616 = t779 * t1843 * t655;
    let t5617 = t2888 * t5616;
    (t5607, t5609, t5612, t5614, t5616, t5617)
}
