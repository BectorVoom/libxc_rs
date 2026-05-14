//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1350/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1350<F: Float>(t113671: F, t1163: F, t33387: F, t113650: F, t113657: F, t113663: F, t113666: F, t113669: F, t32013: F, t32019: F, t32022: F, t32072: F, t32087: F, t32096: F, t32131: F, t33346: F, t33373: F, t33377: F, t33434: F, t33477: F, t6204: F, t82650: F, t9446: F) -> (F, F) {
    let t113673 = t113671 * t33387 * t1163;
    let t113686 = 0.11111111111111111112e0 * t32022 * t33434 - t113650 - 0.41666666666666666668e-1 * t9446 * t6204 * t32013 * t82650 - 0.20833333333333333334e-1 * t9446 * t113657 + 0.20833333333333333334e-1 * t32019 * t33346 + 0.10416666666666666667e-1 * t9446 * t113663 - 0.73697530864197530861e-3 * t113666 - 0.33163888888888888888e-2 * t113669 - 0.13888888888888888889e-1 * t32087 * t113673 - 0.46296296296296296297e-2 * t33373 * t32131 - 0.13888888888888888889e-1 * t32096 * t33477 - 0.13888888888888888889e-1 * t32019 * t33477 - 0.20833333333333333334e-1 * t33373 * t32072 - 0.8041666666666666667e-2 * t33377 * t32072;
    (t113673, t113686)
}
