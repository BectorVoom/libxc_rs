//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1055/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1055<F: Float>(t2862: F, t31613: F, t319: F, t4965: F, t6360: F, t4139: F, t25271: F, t5330: F, t15460: F, t28491: F, t28494: F, t28529: F, t28531: F, t28784: F, t31364: F, t31368: F, t31372: F, t31376: F, t31554: F, t31562: F, t31566: F, t31570: F, t31575: F, t31580: F, t31585: F) -> (F, F, F, F, F, F) {
    let t31885 = t2862 * t319 * t31613;
    let t31890 = t6360 * t4965;
    let t31891 = t4139 * t31890;
    let t31894 = t25271 * t5330;
    let t31895 = t15460 * t31894;
    let t31914 = -2.0 / 9.0 * t31364 + t31368 / 9.0 + 2.0 / 27.0 * t31372 + 2.0 / 9.0 * t31376 - t31554 / 6.0 + 2.0 / 9.0 * t28491 - t28494 / 18.0 + t28529 / 9.0 - 2.0 / 27.0 * t28531 - t31562 / 18.0 - 2.0 / 9.0 * t31566 - t28784 / 27.0 - 4.0 / 9.0 * t31570 - t31575 - t31580 / 8.0 + t31585 / 12.0;
    (t31885, t31890, t31891, t31894, t31895, t31914)
}
