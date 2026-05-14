//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1437/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1437<F: Float>(t115725: F, t3936: F, t109627: F, t115667: F, t4401: F, t109518: F, t109626: F, t114722: F, t114725: F, t114733: F, t114736: F, t114738: F, t114755: F, t114758: F, t114761: F, t114764: F, t115108: F, t115722: F, t21531: F, t32354: F, t33837: F, t9536: F) -> (F, F) {
    let t115890 = t3936 * t115725;
    let t115895 = t109627 * t115667 * t4401;
    let t115911 = -0.34822083333333333332e-2 * t114722 + 0.23214722222222222222e-2 * t114725 - 0.34722222222222222222e-2 * t109626 * t115722 - 0.69444444444444444444e-2 * t109626 * t115890 * t21531 - 0.34722222222222222222e-2 * t109626 * t115895 - 0.61905925925925925925e-2 * t114733 - 0.38691203703703703703e-3 * t114736 - 0.41270617283950617284e-2 * t114738 + 0.15476481481481481481e-2 * t114755 + 0.77382407407407407407e-3 * t114758 + 0.12897067901234567901e-2 * t114761 - 0.23214722222222222222e-2 * t114764 - 0.10416666666666666667e-1 * t32354 * t33837 - 0.40208333333333333334e-2 * t109518 * t33837 + 0.31250000000000000001e-1 * t9536 * t115108;
    (t115895, t115911)
}
