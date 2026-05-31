//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 567/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk567<F: Float>(t1704: F, t1709: F, t1723: F, t1730: F, t1747: F, t1750: F, t1756: F, t1761: F, t1766: F, t1770: F, t236: F, t2759: F, t2761: F, t2769: F, t3137: F, t3142: F, t598: F, t951: F) -> F {
    let t3147 = -t1704 + t1709 + t1723 + t1730 + t1747 + t1750 - t1756 - t1761 - F::cast_from(0.675260332e-1_f64) * t3137 * t598 - F::cast_from(0.1350520664e0_f64) * t951 * t2769 + F::cast_from(0.5848223622634646207e0_f64) * t3142 * t236 + t1766 - t1770 - F::cast_from(0.40020429009866666666e-2_f64) * t2759 + F::cast_from(8.0_f64) * t2761;
    t3147
}
