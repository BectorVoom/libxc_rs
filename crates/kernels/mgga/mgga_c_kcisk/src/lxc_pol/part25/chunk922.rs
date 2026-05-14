//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 922/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk922<F: Float>(t16307: F, t16308: F, t10690: F, t2394: F, t5416: F, t5412: F, t6856: F, t2382: F, t4705: F, t4742: F, t10557: F, t6807: F, t10710: F, t6839: F, t1663: F, t6835: F) -> (F, F, F, F, F, F, F) {
    let t16309 = t16307 * t16308;
    let t16312 = t10690 * t2394;
    let t16313 = t16312 * t5416;
    let t16316 = t6856 * t5412;
    let t16319 = t2382 * t4705;
    let t16321 = 6.0 * t4742 * t16319;
    let t16323 = 4.0 * t10557 * t6807;
    let t16325 = 0.32163648644302209644e2 * t10710 * t6839;
    let t16326 = t6835 * t1663;
    (t16309, t16313, t16316, t16321, t16323, t16325, t16326)
}
