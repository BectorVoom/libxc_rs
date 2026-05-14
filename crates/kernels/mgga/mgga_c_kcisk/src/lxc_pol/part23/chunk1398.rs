//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1398/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1398<F: Float>(t21341: F, t2732: F, t4170: F, t33702: F, t4165: F, t14287: F, t33640: F, t110120: F, t113557: F, t113563: F, t113565: F, t113568: F, t113570: F, t113575: F, t114811: F, t15084: F, t15087: F, t15094: F, t1620: F, t22151: F, t2748: F, t32336: F, t33705: F, t4535: F, t4565: F, t57158: F, t6604: F, t6607: F, t9557: F, t9891: F) -> (F, F, F, F) {
    let t114815 = 2.0 * t4170 * t2732 * t21341;
    let t114817 = 2.0 * t4165 * t33702;
    let t114822 = 4.0 * t14287 * t33640;
    let t114825 = -12.0 * t15094 * t1620 * t33705 + 2.0 * t4535 * t4565 * t9891 + 4.0 * t110120 * t6607 - t15084 * t9891 + 4.0 * t15087 * t33705 - t22151 * t9557 - t2748 * t57158 - t32336 * t6604 - t113557 - t113563 + t113565 + t113568 - t113570 - t113575 - t114811 - t114815 + t114817 - t114822;
    (t114815, t114817, t114822, t114825)
}
