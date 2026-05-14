//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 651/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk651<F: Float>(t7315: F, t7317: F, t6973: F, t740: F, t1950: F, t1941: F, t2560: F, t2580: F, t5310: F, t1949: F, t2586: F, t1948: F, t1940: F, t2567: F, t734: F, t5322: F, t6689: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7318 = t7315 * t7317;
    let t7320 = t6973 * t740;
    let t7321 = t7320 * t1950;
    let t7323 = t2560 * t1941;
    let t7325 = t5310 * t2580;
    let t7327 = t2586 * t1949;
    let t7328 = t1948 * t7327;
    let t7330 = t2567 * t1940;
    let t7331 = t734 * t7330;
    let t7333 = t5322 * t6689;
    (t7318, t7320, t7321, t7323, t7325, t7327, t7328, t7330, t7331, t7333)
}
