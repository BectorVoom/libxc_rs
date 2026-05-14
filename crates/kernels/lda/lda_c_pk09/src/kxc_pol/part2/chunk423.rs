//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 423/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk423<F: Float>(t1000: F, t1001: F, t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F, t995: F, t996: F, t101: F, t89: F, t1069: F, t1071: F, t1073: F, t1100: F, t1108: F, t1114: F, t1115: F, t1116: F, t1117: F, t2248: F, t2252: F, t2256: F, t2260: F, t2264: F, t98: F) -> (F, F, F, F) {
    let t2392 = t995 + t996 + 2.2984542076810275 * t2159 + 2.2984542076810275 * t2163 - 2.2984542076810275 * t2167 + t1000 + t1001 + 0.15282509383508946 * t2171 + 0.15282509383508946 * t2175 - 0.15282509383508946 * t2179;
    let t2393 = t101 * t2392;
    let t2394 = t2393 * t89;
    let t2405 = -t2394 * t98 / 6.0 - 0.10237773105191754 * t2171 - 0.10237773105191754 * t2175 + t1069 + t1071 - t1073 - t1100 + t1108 - 0.14975624337724558 * t2248 - 0.14975624337724558 * t2252 + 0.10237773105191754 * t2179 - 0.01233429741534199 * t2256 - 0.01233429741534199 * t2260 + 0.01233429741534199 * t2264 - t1114 - t1115 - t1116 - t1117;
    (t2392, t2393, t2394, t2405)
}
