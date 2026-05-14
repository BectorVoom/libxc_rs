//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1008/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1008<F: Float>(t44: F, t471: F, t97: F, t9880: F, t8551: F, t8554: F, t4703: F, t4721: F, t4880: F, t4882: F, t4887: F, t4891: F, t4897: F, t4901: F, t4964: F, t4967: F, t2466: F, t3002: F, t48: F, t4938: F, t9858: F, t9864: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t9882 = t97 * t471 * t9880;
    let t9883 = 3.0 * t9882;
    let t9884 = 0.17544670867903938621e1 * t8551;
    let t9885 = 0.54934341918019635162e-3 * t8554;
    let t9886 = t4880 + t4882 - t4887 - t4891 + t4703 + t4897 - t9884 + t4901 - t9885 + t4721 - t4964 + t4967;
    let t9894 = piecewise3(t45, 0.0, -8.0 / 27.0 * t4938 * t9858 + 4.0 / 3.0 * t2466 * t3002 + 4.0 / 3.0 * t48 * t9864);
    (t9883, t9884, t9885, t9886, t9894)
}
