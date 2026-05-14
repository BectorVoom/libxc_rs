//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 993/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk993<F: Float>(t119941: F, t120067: F, t120071: F, t120074: F, t120088: F, t120091: F, t120107: F, t120112: F, t120115: F, t120118: F, t120119: F, t120133: F, t126340: F, t126345: F, t126358: F, t126365: F, t27267: F, t27317: F, t31787: F, t31812: F, t31824: F, t32426: F, t33704: F, t33707: F, t34075: F, t8649: F, t886: F) -> (F,) {
    let t126367 = 0.11423947533020470523e1 * t34075 * t31824 + 0.28234466758480466999e-3 * t126340 + t120067 + 0.3718732920905101082e-3 * t126345 + t120071 - 0.34271842599061411569e1 * t8649 * t31812 * t33707 * t886 - 0.11423947533020470523e1 * t32426 * t33704 - 0.17347256376410398924e1 * t31787 * t27267 - t120074 + 0.34694512752820797848e1 * t119941 * t27317 - 0.1859366460452550541e-3 * t126358 - t120088 - 0.14456046980341999104e-1 * t120091 + 0.66119071333692697238e-4 * t120107 - t120112 + t120115 - t120118 - 0.3718732920905101082e-4 * t120119 - t120133 - 0.28234466758480466999e-3 * t126365;
    (t126367,)
}
