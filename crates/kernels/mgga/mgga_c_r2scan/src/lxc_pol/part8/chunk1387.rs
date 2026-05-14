//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1387/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1387<F: Float>(t18869: F, t18872: F, t18875: F, t18878: F, t18888: F, t18894: F, t26882: F, t26886: F, t32078: F, t32087: F, t32088: F, t10265: F, t759: F, t761: F, t18908: F, t18916: F, t18920: F, t18922: F, t22603: F, t23708: F, t23711: F, t23715: F, t28933: F, t32093: F) -> (F, F) {
    let t33724 = t32078 + t18869 - t18872 - t18875 - t18878 - t26882 - t32087 - t18888 - t32088 - t18894 - 0.5143752e0 * t26886;
    let t33727 = t759 * t10265 * t761;
    let t33731 = t23708 - t32093 - t23711 - t18908 + 0.285764e-1 * t33727 - 0.1714584e0 * t28933 - t23715 - t18916 - 0.675260332e-1 * t22603 - t18920 + t18922;
    (t33724, t33731)
}
