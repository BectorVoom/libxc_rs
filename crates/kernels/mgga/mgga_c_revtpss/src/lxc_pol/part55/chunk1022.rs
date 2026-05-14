//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1022/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1022<F: Float>(t25296: F, t32478: F, t136: F, t2457: F, t8651: F, t31837: F, t93189: F, t120000: F, t32471: F, t119816: F, t1949: F, t28425: F, t25331: F, t32481: F, t119842: F, t2453: F, t25301: F, t32477: F) -> (F, F, F, F, F, F, F, F, F) {
    let t121830 = t32478 * t25296;
    let t121834 = t8651 * t136 * t2457;
    let t121836 = 0.50779446784275991476e-2 * t93189 * t31837 * t121834;
    let t121838 = t120000 * t31837 * t32471;
    let t121840 = 0.39666484489654411541e-3 * t119816;
    let t121846 = t28425 * t1949;
    let t121851 = 0.34270468708064099208e-1 * t32481 * t25331;
    let t121855 = 0.98339826130601561944e-2 * t119842;
    let t121869 = 0.3427046870806409921e-2 * t2453 * t32477 * t25301;
    (t121830, t121834, t121836, t121838, t121840, t121846, t121851, t121855, t121869)
}
