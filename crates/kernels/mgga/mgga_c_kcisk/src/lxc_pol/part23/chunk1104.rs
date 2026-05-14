//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1104/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1104<F: Float>(t22206: F, t22221: F, t44: F, t1322: F, t2168: F, t1284: F, t2231: F, t1591: F, t2326: F, t2753: F, t3465: F, t167: F, t3233: F, t3236: F, t9345: F, t1032: F, t967: F) -> (F, F, F, F, F, F, F, F) {
    let t22223 = (t22206 + t22221) * t44;
    let t26035 = t2168 * t1322;
    let t26773 = t1284 * t2231;
    let t27725 = t2326 * t1591;
    let t31823 = t3465 * t2753;
    let t31824 = t31823 / 8.0;
    let t31825 = t3233 * t167;
    let t31827 = t3236 * t9345;
    let t31829 = t1032 * t967;
    (t22223, t26035, t26773, t27725, t31824, t31825, t31827, t31829)
}
