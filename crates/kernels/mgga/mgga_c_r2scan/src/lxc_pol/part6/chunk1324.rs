//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1324/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1324<F: Float>(t18888: F, t18908: F, t18916: F, t18920: F, t18922: F, t18930: F, t23700: F, t23701: F, t23702: F, t23705: F, t23706: F, t23707: F, t23708: F, t23710: F, t23711: F, t23715: F, t23718: F) -> (F,) {
    let t25021 = -t23700 + t23701 + t23702 + t18888 + t23705 + t23706 + t23707 + t23708 + t23710 - t23711 + t18908 - t23715 + t18916 + t18920 - t18922 - t18930 - t23718;
    (t25021,)
}
