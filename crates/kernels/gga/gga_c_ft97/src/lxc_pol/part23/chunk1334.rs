//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1334/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1334<F: Float>(t2842: F, t7021: F, t1882: F, t31783: F, t10703: F, t114055: F, t114062: F, t114222: F, t114578: F, t11593: F, t125742: F, t1508: F, t15312: F, t15369: F, t15460: F, t1901: F, t19333: F, t19362: F, t19367: F, t19867: F, t19872: F, t24873: F, t2862: F, t296: F, t31702: F, t31956: F, t4176: F, t4181: F, t4299: F, t446: F, t5413: F, t56854: F, t6287: F, t6353: F, t824: F, t840: F, t871: F) -> (F,) {
    let t126613 = t2842 * t7021;
    let t126643 = t1882 * t31783;
    let t126645 = -2.0 / 9.0 * t1901 * t10703 * t24873 * t19867 + 4.0 / 9.0 * t11593 * t10703 * t24873 * t19872 - 4.0 / 9.0 * t1901 * t56854 * t31702 - 4.0 / 9.0 * t1901 * t15312 * t114222 * t5413 + 4.0 / 3.0 * t446 * t296 * t125742 - t114055 - 4.0 / 3.0 * t1901 * t15460 * t126613 * t4181 - t114062 + t446 * t840 * t6353 * t19367 / 3.0 + 2.0 / 3.0 * t446 * t840 * t871 * t7021 * t4299 - t446 * t840 * t31956 * t824 / 3.0 - 4.0 / 3.0 * t1901 * t15369 * t114578 * t4176 + t446 * t840 * t19333 * t6287 / 3.0 + 2.0 / 3.0 * t446 * t2862 * t1508 * t19362 - 2.0 / 9.0 * t126643;
    (t126645,)
}
