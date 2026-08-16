//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1241/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1241<F: Float>(t792: F, t9573: F, t3275: F, t3276: F, t11550: F, t983: F, t3262: F, t43764: F, t43766: F, t43770: F, t43774: F, t43778: F, t43780: F, t43782: F, t43783: F, t43785: F, t43787: F, t43789: F, t43791: F, t43795: F, t43797: F) -> (F, F, F) {
    let t43798 = t9573 * t792;
    let t43801 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t3276 * t43798;
    let t43802 = t11550 * t983;
    let t43805 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t3276 * t43802;
    let t43806 = -t43764 + t43766 + t43770 + t43774 - t43778 - t43780 + t43782 + t43783 - t43785 - t43787 - t43789 + t43791 + t43795 + t43797 - t43801 - t43805;
    (t43801, t43805, t43806)
}
