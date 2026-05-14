//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1213/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1213<F: Float>(t29064: F, t8392: F, t4181: F, t98724: F, t1882: F, t29235: F, t7042: F, t8232: F, t2842: F, t6260: F, t309: F, t43524: F, t29270: F, t10703: F, t11593: F, t15312: F, t15369: F, t15403: F, t15407: F, t15425: F, t15441: F, t15446: F, t15460: F, t15472: F, t15477: F, t1901: F, t24873: F, t24886: F, t2665: F, t29055: F, t29071: F, t29093: F, t29203: F, t296: F, t319: F, t3281: F, t446: F, t57032: F, t6273: F, t6334: F) -> (F, F) {
    let t112853 = 2.0 / 27.0 * t8392 * t29064;
    let t112860 = t98724 * t4181;
    let t112865 = 2.0 / 9.0 * t1882 * t29235;
    let t112866 = t8232 * t7042;
    let t112883 = t2842 * t6260;
    let t112888 = t43524 * t309;
    let t112898 = 4.0 / 9.0 * t1882 * t29270;
    let t112903 = -t112853 + 4.0 / 9.0 * t1901 * t29093 * t15403 - 8.0 / 9.0 * t11593 * t24886 * t15446 + 4.0 / 3.0 * t446 * t296 * t112860 + t112865 - 4.0 / 27.0 * t112866 - 4.0 / 9.0 * t1901 * t57032 * t29203 + 4.0 / 9.0 * t11593 * t10703 * t24873 * t15407 + 8.0 / 9.0 * t11593 * t15312 * t24873 * t15441 - 2.0 * t1901 * t29071 * t6273 * t15425 - 4.0 / 3.0 * t1901 * t15460 * t112883 * t4181 + 8.0 * t1901 * t112888 * t6273 * t15477 + 4.0 / 3.0 * t1901 * t15369 * t29055 * t15472 - t112898 - 2.0 / 9.0 * t3281 * t2665 * t319 * t6334;
    (t112860, t112903)
}
