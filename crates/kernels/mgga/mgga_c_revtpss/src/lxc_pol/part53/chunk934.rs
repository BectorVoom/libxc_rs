//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 934/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk934<F: Float>(t4248: F, t8460: F, t7889: F, t7742: F, t8634: F, t4147: F, t7933: F, t2034: F, t2014: F, t7937: F, t8568: F, t32098: F, t7900: F, t7901: F, t33639: F, t508: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33643 = t4248 * t8460;
    let t33644 = 2.0 * t33643;
    let t33645 = t7889 * t8460;
    let t33646 = 2.0 * t33645;
    let t33650 = 4.0 * t8634 * t7742;
    let t33651 = t4147 * t7933;
    let t33652 = t2034 * t33651;
    let t33654 = 2.0 * t2014 * t33652;
    let t33655 = t8568 * t7937;
    let t33657 = t32098 * t7900;
    let t33659 = 3.0 * t2014 * t33657;
    let t33661 = t8568 * t7901;
    let t33664 = 2.0 * t33639 * t508;
    (t33643, t33644, t33645, t33646, t33650, t33651, t33652, t33654, t33655, t33657, t33659, t33661, t33664)
}
