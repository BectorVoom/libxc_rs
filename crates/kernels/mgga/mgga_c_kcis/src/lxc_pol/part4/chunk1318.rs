//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1318/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1318<F: Float>(t16919: F, t3984: F, t12147: F, t5722: F, t1368: F, t531: F, t5732: F, t833: F, t5705: F, t12135: F, t12138: F, t12142: F, t12152: F, t16902: F, t16907: F, t16911: F, t3986: F, t5691: F) -> F {
    let t16920 = t3984 * t16919;
    let t16923 = t12147 * t5722;
    let t16925 = t1368 * t16923 / F::new(432.0);
    let t16926 = t5732 * t531;
    let t16927 = t16926 * t833;
    let t16928 = t3984 * t16927;
    let t16933 = t12147 * t5705;
    let t16935 = t1368 * t16933 / F::new(432.0);
    let t16936 = F::new(7.0) / F::new(648.0) * t1368 * t16902 - t1368 * t16907 / F::new(54.0) - t1368 * t16911 / F::new(288.0) - t12135 / F::new(648.0) + t12138 / F::new(864.0) + t12142 / F::new(648.0) - t12152 / F::new(432.0) + t1368 * t16920 / F::new(144.0) - t16925 - t1368 * t16928 / F::new(144.0) + t5691 * t3986 / F::new(54.0) - t16935;
    t16936
}
