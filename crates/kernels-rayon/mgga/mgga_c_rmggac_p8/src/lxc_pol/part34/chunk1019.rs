//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1019/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1019(t77723: f64, t75662: f64, t75664: f64, t71544: f64, t71545: f64, t71546: f64, t71551: f64, t75648: f64, t75652: f64, t75654: f64, t75658: f64, t75666: f64, t77713: f64, t77715: f64, t77717: f64, t77719: f64, t77720: f64, t884: f64) -> f64 {
    let t77724 = 0.99317399751028291929e-5_f64 * t77723;
    let t77725 = 0.3830813990396805546e-4_f64 * t75662;
    let t77726 = 0.1276937996798935182e-4_f64 * t75664;
    let t77727 = 0.93188427318671584245e-2_f64 * t75648 + 0.93188427318671584245e-2_f64 * t75652 - 0.15531404553111930708e-1_f64 * t75654 - 0.15531404553111930708e-1_f64 * t75658 + t77713 + t77715 + t77717 - t77719 + 0.59871208509319042821e-1_f64 * t884 * t77720 + t77724 + t77725 + t77726 + t75666 - t71544 - t71545 + t71546 + t71551;
    t77727
}
