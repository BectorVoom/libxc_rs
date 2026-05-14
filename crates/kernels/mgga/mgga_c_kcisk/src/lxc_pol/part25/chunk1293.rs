//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1293/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1293<F: Float>(t2063: F, t33032: F, t5043: F, t7242: F, t17345: F, t1790: F, t220: F, t10879: F, t9664: F, t9935: F, t2469: F, t36267: F, t5032: F, t7261: F, t17182: F, t34147: F) -> (F, F, F, F, F) {
    let t116448 = t7242 * t33032 * t2063 * t5043;
    let t116453 = t17345 * t33032 * t220 * t1790;
    let t116465 = t9664 * t10879 * t9935;
    let t116469 = t7261 * t36267 * t2469 * t5032;
    let t116474 = t17182 * t34147;
    (t116448, t116453, t116465, t116469, t116474)
}
