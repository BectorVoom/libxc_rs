//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 814/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk814<F: Float>(t587: F, t7663: F, t2555: F, t5125: F, t197: F, t5283: F, t2561: F, t1000: F, t1866: F, t1827: F, t1821: F, t7350: F, t2559: F, t7326: F, t5359: F, t7634: F, t7636: F, t7637: F, t7639: F, t7644: F, t7648: F, t7650: F, t7655: F, t7658: F, t7662: F) -> (F, F, F, F, F, F, F) {
    let t7665 = 16.0 / 135.0 * t587 * t7663;
    let t7666 = t5125 * t2555;
    let t7668 = 32.0 / 135.0 * t587 * t7666;
    let t7669 = t5283 * t197;
    let t7670 = t7669 * t2561;
    let t7672 = 16.0 / 81.0 * t587 * t7670;
    let t7673 = t1000 * t1866;
    let t7674 = t1827 * t7673;
    let t7676 = 4.0 / 45.0 * t587 * t7674;
    let t7677 = t1821 * t7350;
    let t7679 = 8.0 / 45.0 * t587 * t7677;
    let t7680 = t2559 * t7326;
    let t7682 = 8.0 / 9.0 * t587 * t7680;
    let t7683 = t7634 - t7636 - t7637 + t5359 + t7639 - t7644 + t7648 + t7650 - t7655 + t7658 + t7662 - t7665 - t7668 + t7672 - t7676 - t7679 - t7682;
    (t7665, t7668, t7672, t7676, t7679, t7682, t7683)
}
