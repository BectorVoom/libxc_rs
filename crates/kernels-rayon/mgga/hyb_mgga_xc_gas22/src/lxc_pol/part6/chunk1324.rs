//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1324/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1324(t10571: f64, t2200: f64, t10584: f64, t3335: f64, t8682: f64, t10577: f64, t2206: f64, t791: f64, t10590: f64, t3329: f64, t8672: f64, t28877: f64, t28880: f64, t28883: f64, t28885: f64, t28887: f64, t28890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28892 = t10571 * t2200;
    let t28894 = t10584 * t2200;
    let t28896 = t3335 * t8682;
    let t28899 = t2206 * t10577 * t791;
    let t28901 = t10590 * t2200;
    let t28903 = t791 * t3329;
    let t28904 = t8672 * t28903;
    let t28906 = 0.6189328125e-1_f64 * t28877 - 0.412621875e-1_f64 * t28880 - 0.485484375e1_f64 * t28883 + 0.19419375e1_f64 * t28885 - 0.258925e1_f64 * t28887 - 0.258925e1_f64 * t28890 - 0.1294625e1_f64 * t28892 - 0.412621875e-1_f64 * t28894 + 0.16504875e0_f64 * t28896 + 0.16504875e0_f64 * t28899 + 0.82524375e-1_f64 * t28901 - 0.16504875e0_f64 * t28904;
    (t28892, t28894, t28896, t28899, t28901, t28903, t28904, t28906)
}
