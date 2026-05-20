//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1334/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1334<F: Float>(t550: F, t5658: F, t543: F, t3992: F, t2661: F, t5610: F, t9775: F, t1889: F, t9779: F, t828: F, t9954: F, t1398: F, t1868: F) -> (F, F, F, F, F, F) {
    let t13774 = t550 * t5658;
    let t13775 = t13774 * t543;
    let t13776 = t3992 * t13775;
    let t13778 = F::cast_from(0.14291339372689912324e-4_f64) * t2661 * t13776;
    let t13779 = t9775 * t5610;
    let t13781 = t9779 * t1889;
    let t13783 = t9954 * t828;
    let t13784 = t1868 * t1398;
    (t13775, t13778, t13779, t13781, t13783, t13784)
}
