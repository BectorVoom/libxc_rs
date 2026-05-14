//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 203/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk203<F: Float>(t1035: F, t313: F, t1029: F, t531: F, t1022: F, t808: F, t568: F, t836: F, t317: F, t797: F, t813: F, t833: F, t960: F, t971: F) -> (F, F, F, F, F, F, F) {
    let t1036 = t313 * t1035;
    let t1040 = t531 * t1029;
    let t1043 = t808 * t1022;
    let t1044 = t568 * t1043;
    let t1048 = t836 * t1022;
    let t1049 = t568 * t1048;
    let t1052 = 0.35750489951850426669e0 * t1036 * t317 + 0.29792074959875355558e-1 * t960 - 0.35750489951850426669e0 * t797 * t1040 - 0.23005755572352449806e1 * t813 * t1044 - 0.19171462976960374838e0 * t971 + 0.23005755572352449806e1 * t833 * t1049;
    (t1036, t1040, t1043, t1044, t1048, t1049, t1052)
}
