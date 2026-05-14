//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1258/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1258<F: Float>(t1501: F, t55792: F, t10696: F, t1476: F, t29084: F, t8392: F, t1882: F, t29389: F, t7111: F, t8232: F, t44600: F, t112426: F, t14602: F, t14607: F, t15195: F, t15312: F, t15460: F, t1901: F, t19500: F, t2405: F, t2413: F, t25276: F, t25280: F, t25369: F, t28843: F, t28847: F, t29055: F, t29128: F, t296: F, t446: F, t56819: F, t684: F, t7114: F, t824: F, t840: F) -> (F, F) {
    let t113843 = t55792 * t1501;
    let t113847 = t10696 * t1476;
    let t113856 = 2.0 / 27.0 * t8392 * t29084;
    let t113866 = 2.0 / 9.0 * t1882 * t29389;
    let t113867 = t8232 * t7111;
    let t113869 = t44600 * t1501;
    let t113895 = -t446 * t296 * t113843 / 3.0 + 2.0 * t1901 * t15460 * t113847 * t14602 - 2.0 / 3.0 * t446 * t296 * t112426 - t113856 - 2.0 / 3.0 * t1901 * t15460 * t29055 * t14607 - 2.0 / 3.0 * t446 * t840 * t28843 * t824 + t113866 - 4.0 / 27.0 * t113867 + 8.0 * t1901 * t29128 * t113869 * t14602 + 2.0 / 27.0 * t1901 * t19500 * t25280 + t1901 * t15195 * t25276 / 9.0 + 2.0 / 9.0 * t1901 * t15195 * t25369 - 4.0 / 9.0 * t1901 * t15312 * t28847 * t684 - 2.0 / 9.0 * t1901 * t15312 * t7114 * t2413 - 4.0 / 27.0 * t1901 * t56819 * t7114 * t2405;
    (t113843, t113895)
}
