//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 972/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk972<F: Float>(t10570: F, t11371: F, t15989: F, t15993: F, t15996: F, t16061: F, t22603: F, t22605: F, t22608: F, t22610: F, t22628: F, t11352: F, t11355: F, t16037: F, t16040: F, t1648: F, t1815: F, t22542: F, t22547: F, t22556: F, t22596: F, t22599: F, t2372: F, t4664: F, t4667: F, t574: F, t6750: F, t6771: F, t6774: F, t8504: F, t8522: F) -> (F,) {
    let t22629 = 0.14865e-1 * t22603 - 0.1982e-1 * t22605 - 0.991e-2 * t22608 + 0.1982e-1 * t22610 - t11371 - 0.18344444444444444444e-2 * t10570 - 0.36688888888888888888e-2 * t15989 + t16061 - 0.55033333333333333332e-2 * t15993 - 0.55033333333333333332e-2 * t15996 + t22628;
    let t22632 = 3.0 / 16.0 * t11352 * t22542 - t11355 * t8504 / 8.0 - t4664 * t22547 / 4.0 - t16037 * t6750 / 4.0 + t16040 * t2372 / 2.0 + t6774 * t6771 / 2.0 - t4664 * t22556 / 8.0 + t4667 * t8522 / 4.0 + t1815 * t22596 / 4.0 + t22599 * t1648 / 4.0 + t574 * t22629 / 2.0;
    (t22632,)
}
