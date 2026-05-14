//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1209/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1209<F: Float>(t668: F, t7124: F, t2405: F, t7036: F, t10478: F, t1508: F, t25188: F, t848: F, t29190: F, t8392: F, t10703: F, t14686: F, t15183: F, t15308: F, t15313: F, t1901: F, t2413: F, t24873: F, t24874: F, t24878: F, t2876: F, t28760: F, t29189: F, t29208: F, t29215: F, t29302: F, t44369: F, t56098: F, t56352: F, t56643: F, t56647: F, t56854: F, t57180: F, t684: F, t7105: F, t72163: F, t99238: F) -> (F, F, F) {
    let t112725 = t7124 * t668;
    let t112742 = t7036 * t2405;
    let t112746 = t10478 * t1508;
    let t112760 = t848 * t25188;
    let t112765 = 4.0 / 27.0 * t8392 * t29190;
    let t112766 = -4.0 / 9.0 * t1901 * t57180 * t28760 - 2.0 / 9.0 * t1901 * t10703 * t29302 * t684 + 2.0 / 3.0 * t1901 * t56352 * t24873 * t15183 - 2.0 / 9.0 * t1901 * t10703 * t112725 * t2876 - 4.0 / 9.0 * t1901 * t56854 * t29189 + 4.0 / 27.0 * t1901 * t56647 * t29208 - 2.0 / 9.0 * t1901 * t56098 * t24874 - 4.0 / 9.0 * t1901 * t72163 * t24878 - 4.0 / 27.0 * t1901 * t56643 * t112742 + 4.0 / 27.0 * t1901 * t112746 * t14686 - t1901 * t10703 * t7105 * t2413 / 9.0 - 2.0 / 9.0 * t1901 * t44369 * t29215 - 2.0 / 9.0 * t1901 * t99238 * t15308 - 4.0 / 9.0 * t1901 * t112760 * t15313 + t112765;
    (t112725, t112742, t112766)
}
