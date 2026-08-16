//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 568/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk568(t1704: f64, t1709: f64, t1723: f64, t1730: f64, t1747: f64, t1750: f64, t1756: f64, t1761: f64, t1766: f64, t1770: f64, t236: f64, t2759: f64, t2761: f64, t2769: f64, t3137: f64, t3142: f64, t598: f64, t951: f64) -> f64 {
    let t3147 = -t1704 + t1709 + t1723 + t1730 + t1747 + t1750 - t1756 - t1761 - 0.675260332e-1_f64 * t3137 * t598 - 0.1350520664e0_f64 * t951 * t2769 + 0.5848223622634646207e0_f64 * t3142 * t236 + t1766 - t1770 - 0.40020429009866666666e-2_f64 * t2759 + 8.0_f64 * t2761;
    t3147
}
