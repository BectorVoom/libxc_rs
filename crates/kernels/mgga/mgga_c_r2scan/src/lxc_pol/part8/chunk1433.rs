//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1433/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1433<F: Float>(t113: F, t32523: F, t19865: F, t6086: F, t10308: F, t783: F, t784: F, t788: F, t22985: F, t23018: F, t23025: F, t2721: F, t27246: F, t27950: F, t31178: F, t31182: F, t31184: F, t31187: F, t31190: F, t31206: F, t944: F, t9464: F) -> (F,) {
    let t34717 = t32523 * t113;
    let t34719 = t19865 * t6086 * t34717;
    let t34732 = t783 * t10308 * t784 * t788;
    let t34735 = 0.19634394786159580878e0 * t22985 - 0.34930954652346593433e-1 * t34719 + t27246 - 0.34672886960217074253e0 * t31178 - 0.13002332610081402845e0 * t9464 * t2721 - 0.39006997830244208535e0 * t27950 * t944 + 0.15256070262495512671e2 * t31182 - 0.11708928647259339622e0 * t31184 - 0.69861909304693186866e-1 * t31187 - 0.2095857279140795606e0 * t31190 + 0.58218257753910989057e-2 * t34732 - t23018 + t23025 + 0.98781737744032673978e-1 * t31206;
    (t34735,)
}
