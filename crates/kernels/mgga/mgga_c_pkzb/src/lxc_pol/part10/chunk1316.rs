//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1316/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1316<F: Float>(t25782: F, t25785: F, t25788: F, t25790: F, t25793: F, t25795: F, t25797: F, t25799: F, t25802: F, t25804: F, t25807: F, t25809: F, t25982: F, t25995: F, t26008: F, t713: F, t722: F, t730: F) -> (F, F) {
    let t26021 = -0.485484375e1 * t25782 + 0.19419375e1 * t25785 + 0.6189328125e-1 * t25788 - 0.258925e1 * t25790 - 0.258925e1 * t25793 - 0.1294625e1 * t25795 - 0.412621875e-1 * t25797 + 0.16504875e0 * t25799 + 0.16504875e0 * t25802 + 0.82524375e-1 * t25804 - 0.412621875e-1 * t25807 + 0.19419375e1 * t25809;
    let t26023 = t25982 + t25995 + t26008 + t26021;
    let t26027 = 0.5848223622634646207e0 * t730 * t713 * t26023 * t722;
    (t26023, t26027)
}
