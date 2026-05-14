//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1052/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1052<F: Float>(t12: F, t19822: F, t496: F, t6825: F, t2562: F, t500: F, t1064: F, t19633: F, t19636: F, t19642: F, t19645: F, t207: F, t2732: F, t2735: F, t439: F, t5094: F, t5100: F, t7337: F, t7340: F, t7345: F, t82: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t19823 = 0.51947577317044391276e2 * t19822;
    let t19824 = t496 * t6825;
    let t19825 = 12.0 * t19824;
    let t19843 = 16.0 * t2562 * t500;
    let t19845 = piecewise3(t84, 0.0, -56.0 / 81.0 * t7337 * t5094 + 16.0 / 9.0 * t7340 * t19633 + 8.0 / 9.0 * t2732 * t19636 - 4.0 / 3.0 * t7345 * t439 + 4.0 * t2735 * t19642 - 4.0 / 3.0 * t2735 * t19645 - 2.0 / 9.0 * t1064 * t5100 - 8.0 * t207 * t82 + t19843);
    (t19823, t19825, t19845)
}
