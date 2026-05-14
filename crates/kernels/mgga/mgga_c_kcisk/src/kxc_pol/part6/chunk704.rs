//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 704/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk704<F: Float>(t11313: F, t2514: F, t2507: F, t5060: F, t2399: F, t4822: F, t2456: F, t4995: F, t2449: F, t2454: F, t3934: F, t649: F, t164: F, t2465: F, t1773: F, t2448: F, t654: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16897 = t11313 * t2514;
    let t17056 = t2507 * t5060;
    let t17057 = t17056 * sigma2;
    let t17078 = t2399 * t4822;
    let t17220 = t2456 * t4995;
    let t17222 = t2449 * t4995;
    let t17248 = t649 * t2454 * t3934;
    let t17276 = t164 * t2465;
    let t17277 = t1773 * t17276;
    let t17317 = t2448 * t654 * t3934;
    (t16897, t17056, t17057, t17078, t17220, t17222, t17248, t17277, t17317)
}
