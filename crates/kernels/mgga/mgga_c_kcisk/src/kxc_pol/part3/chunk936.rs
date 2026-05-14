//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 936/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk936<F: Float>(t227: F, t3288: F, t1060: F, t3289: F, t10441: F, t10449: F, t229: F, t3293: F, t15781: F, t44: F, t291: F, t15174: F, t15452: F, t15457: F, t15460: F, t15463: F, t15466: F, t15471: F, t15473: F, t15763: F, t15766: F, t15767: F, t15770: F, zeta_threshold: F) -> (F,) {
    let t228 = t227 <= zeta_threshold;
    let t15783 = 1.0 / t3288 / t227;
    let t15786 = t3289 * t1060;
    let t15792 = piecewise3(t228, 0.0, -8.0 / 27.0 * t15783 * t10441 + 4.0 / 3.0 * t15786 * t3293 + 4.0 / 3.0 * t229 * t10449);
    let t15794 = (t15781 + t15792) * t44;
    let t15795 = t15794 * t291;
    let t15796 = -t15174 + t15452 - t15457 - t15460 - t15463 - t15466 - t15471 - t15473 + t15763 - t15766 + 3.0 * t15767 - t15770 + t15795;
    (t15796,)
}
