//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1334;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta324(t2737: f64, t9802: f64, t221: f64, t2485: f64, t2754: f64, t2484: f64, t2749: f64, t836: f64, t853: f64, t2662: f64, t2661: f64, t2646: f64, t2482: f64, t596: f64, t823: f64, t2487: f64, t27: f64, t2719: f64, t2724: f64, t2741: f64, t2756: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10826, t10832, t10833, t10836, t10838, t10841) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1334(t2737, t9802, t221, t2485, t2754, t2484, t2749, t836, t853, t2662, t2661, t2646);
        let (t10842, t10845, t10846, t10852, t10853, t10855) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1335(t10841, t2484, t2482, t596, t823, t2487, t27, t2719, t221, t2485, t2724, t2741, t2756);
    (t10826, t10832, t10833, t10836, t10838, t10841, t10842, t10845, t10846, t10852, t10853, t10855)
}
