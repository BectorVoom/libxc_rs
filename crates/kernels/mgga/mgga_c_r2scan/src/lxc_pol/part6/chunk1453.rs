//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1453/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1453<F: Float>(t18869: F, t18872: F, t18875: F, t18878: F, t23693: F, t23694: F, t23696: F, t23697: F, t23698: F, t23699: F, t23700: F, t18888: F, t23701: F, t23702: F, t23705: F, t23706: F, t23707: F, t23708: F, t23710: F, t23711: F, t23714: F, t8505: F, t885: F) -> (F, F) {
    let t27420 = t23693 + t23694 - t23696 - t23697 - t23698 - t23699 - t18869 + t18872 + t18875 + t18878 - t23700;
    let t27424 = 3.0 * t8505 * t885 + t18888 + t23701 + t23702 + t23705 + t23706 + t23707 + t23708 + t23710 - t23711 + t23714;
    (t27420, t27424)
}
