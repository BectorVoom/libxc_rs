//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 704/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk704<F: Float>(t13822: F, t574: F, t13749: F, t600: F, t568: F, t597: F, t189: F, t188: F, t193: F, t12962: F, t12966: F, t12970: F, t12989: F, t12992: F, t12994: F, t12998: F, t13806: F, t13808: F, t13811: F, t13815: F, t13820: F) -> (F, F, F, F, F) {
    let t13824 = 0.23005755572352449806e1 * t574 * t13822;
    let t13825 = t600 * t13749;
    let t13826 = t568 * t13825;
    let t13828 = 0.23005755572352449806e1 * t597 * t13826;
    let t13829 = t189 * t13749;
    let t13830 = t188 * t13829;
    let t13832 = 0.35750489951850426669e0 * t13830 * t193;
    let t13834 = 0.11502877786176224903e2 * t13806 - 0.10725146985555128001e1 * t13808 + 0.71500979903700853338e0 * t13811 - 0.69017266717057349418e1 * t13815 + t12962 - 0.19171462976960374838e0 * t12966 - t12970 - t13820 - t13824 + t13828 + t13832 + t12989 + t12992 + 0.19171462976960374838e0 * t12994 + t12998;
    (t13825, t13826, t13829, t13830, t13834)
}
