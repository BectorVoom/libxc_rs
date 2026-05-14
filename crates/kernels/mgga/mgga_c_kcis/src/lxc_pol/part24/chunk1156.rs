//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1156/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1156<F: Float>(t26686: F, t4781: F, t4972: F, t101501: F, t7690: F, t19674: F, t283: F, t990: F, t101469: F, t26685: F, t7703: F, t7706: F, t93163: F, t93662: F, t96302: F, t96306: F, t96340: F, t96345: F, t96358: F) -> (F, F) {
    let t101524 = t26686 * t4781 * t4972;
    let t101532 = t7690 * t101501;
    let t101536 = t19674 * t283 * t990;
    let t101539 = -0.18550940104166666667e-3 * t26685 * t101524 - 0.69505208333333333333e-3 * t7703 * t101469 + t96302 - 0.20612155671296296296e-4 * t93662 - 0.30891203703703703704e-3 * t96306 + 0.14739506172839506173e-2 * t93163 + 0.30918233506944444444e-4 * t101532 + t96340 - 0.44218518518518518516e-2 * t96345 + t96358 - 0.23168402777777777778e-3 * t101536 * t7706;
    (t101524, t101539)
}
