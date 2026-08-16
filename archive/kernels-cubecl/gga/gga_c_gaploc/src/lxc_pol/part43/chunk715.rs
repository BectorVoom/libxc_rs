//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 715/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk715<F: Float>(t13821: F, t568: F, t574: F, t13749: F, t600: F, t597: F, t189: F, t188: F, t193: F, t3749: F, t977: F, t1960: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13822 = t568 * t13821;
    let t13824 = F::cast_from(0.23005755572352449806e1_f64) * t574 * t13822;
    let t13825 = t600 * t13749;
    let t13826 = t568 * t13825;
    let t13828 = F::cast_from(0.23005755572352449806e1_f64) * t597 * t13826;
    let t13829 = t189 * t13749;
    let t13830 = t188 * t13829;
    let t13832 = F::cast_from(0.35750489951850426669e0_f64) * t13830 * t193;
    let t13838 = t3749 * t977;
    let t13839 = t1960 * t13838;
    (t13822, t13824, t13825, t13826, t13828, t13829, t13830, t13832, t13838, t13839)
}
