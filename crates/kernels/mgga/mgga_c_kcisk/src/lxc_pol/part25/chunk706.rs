//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 706/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk706<F: Float>(t2021: F, t7638: F, t1586: F, t2005: F, t2013: F, t2638: F, t2644: F, t5471: F, t5484: F, t7603: F, t7606: F, t7611: F, t7615: F, t7619: F, t7625: F, t7629: F, t7634: F, t782: F) -> (F, F, F) {
    let t7639 = t2021 * t7638;
    let t7640 = t1586 * t7639;
    let t7643 = -0.89953943580886586067e-2 * t5484 + 0.89953943580886586067e-2 * t5471 * t2638 + 0.29984647860295528689e-2 * t7603 + 0.11993859144118211476e-1 * t2013 * t7606 - 0.89953943580886586067e-2 * t2013 * t7611 - 0.17990788716177317213e-1 * t2013 * t7615 - 0.17990788716177317213e-1 * t2013 * t7619 - 0.2698618307426597582e-1 * t2005 * t2644 - 0.89953943580886586067e-2 * t7625 - 0.89953943580886586067e-2 * t2013 * t7629 + 0.5397236614853195164e-1 * t2013 * t7634 - 0.2698618307426597582e-1 * t782 * t7640;
    (t7639, t7640, t7643)
}
