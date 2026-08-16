//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1095/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1095<F: Float>(t30714: F, t32844: F, t1516: F, t8343: F, t30698: F, t30705: F, t30722: F, t32835: F, t32838: F, t32841: F, t235: F, t1499: F, t226: F, t30675: F, t30683: F, t32821: F, t32825: F, t32829: F, t32831: F, t812: F, t8360: F) -> (F, F, F) {
    let t32845 = t30714 * t32844;
    let t32847 = t8343 * t1516;
    let t32849 = -t30698 - F::cast_from(0.48447307312968469025e-2_f64) * t32835 - t30705 - F::cast_from(0.80745512188280781708e-3_f64) * t32838 + t32841 / F::cast_from(1536.0_f64) - t32845 / F::cast_from(1536.0_f64) - t30722 - t32847 / F::cast_from(384.0_f64);
    let t32850 = t235 * t32849;
    let t32852 = t1499 * t8360 + t226 * t32850 - t32831 * t812 - t30675 - t30683 - t32821 - t32825 + t32829;
    (t32849, t32850, t32852)
}
