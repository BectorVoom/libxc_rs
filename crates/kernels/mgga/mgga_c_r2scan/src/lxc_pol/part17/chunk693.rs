//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 693/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk693<F: Float>(t1745: F, t732: F, t1731: F, t5311: F, t5314: F, t636: F, t12: F, t3: F, t40: F, t1737: F, t4735: F, t4738: F, t640: F) -> (F, F, F, F, F, F) {
    let t5413 = t732 * t1745;
    let t5416 = t1731 * t5311;
    let t5418 = t636 * t5314;
    let t5420 = F::powf(t12, -F::new(0.25e1));
    let t5421 = t5420 * t3;
    let t5422 = t5421 * t40;
    let t5424 = t1737 * t4735;
    let t5426 = t640 * t4738;
    (t5413, t5416, t5418, t5422, t5424, t5426)
}
