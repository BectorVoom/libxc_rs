//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1260/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1260<F: Float>(t2788: F, t4965: F, t18900: F, t19016: F, t2266: F, t910: F, t18912: F, t18908: F, t18916: F, t18920: F, t19445: F, t23705: F, t23706: F, t23707: F, t23708: F, t18931: F) -> (F, F, F, F, F, F) {
    let t23709 = t2788 * t4965;
    let t23710 = 0.48159733137676571078e0 * t23709;
    let t23711 = 48.0 * t18900;
    let t23714 = 6.0 * t2266 * t19016 * t910;
    let t23715 = 960.0 * t18912;
    let t23717 = -t23705 - t23706 - t23707 - t23708 - t23710 + t23711 - t23714 - t18908 + t23715 - t18916 - t18920 - 0.7089e1 * t19445;
    let t23718 = 0.5848223622634646207e0 * t18931;
    (t23710, t23711, t23714, t23715, t23717, t23718)
}
