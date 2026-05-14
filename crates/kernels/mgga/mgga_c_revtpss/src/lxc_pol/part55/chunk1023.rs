//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1023/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1023<F: Float>(t121803: F, t1955: F, t119869: F, t25331: F, t32478: F, t2470: F, t32470: F, t32474: F, t119808: F, t8477: F, t32469: F, t121834: F, t31837: F, t93169: F, t119903: F, t121808: F, t31830: F) -> (F, F, F, F, F, F, F, F, F) {
    let t121870 = t1955 * t121803;
    let t121879 = 0.35702867204846465857e-4 * t119869;
    let t121881 = 0.19274729307122665472e-1 * t32478 * t25331;
    let t121884 = t32470 * t2470;
    let t121886 = 0.33852964522850660984e-1 * t32474 * t121884;
    let t121887 = t8477 * t119808;
    let t121891 = 0.19039912555034117539e-1 * t32469 * t121884;
    let t121896 = 0.95199562775170587692e-3 * t93169 * t31837 * t121834;
    let t121897 = 0.37645955677973955999e-5 * t119903;
    let t121901 = t31830 * t121808;
    (t121870, t121879, t121881, t121886, t121887, t121891, t121896, t121897, t121901)
}
