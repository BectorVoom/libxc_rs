//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1324/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1324<F: Float>(t10571: F, t2200: F, t10584: F, t3335: F, t8682: F, t10577: F, t2206: F, t791: F, t10590: F, t3329: F, t8672: F, t28877: F, t28880: F, t28883: F, t28885: F, t28887: F, t28890: F) -> (F, F, F, F, F, F, F, F) {
    let t28892 = t10571 * t2200;
    let t28894 = t10584 * t2200;
    let t28896 = t3335 * t8682;
    let t28899 = t2206 * t10577 * t791;
    let t28901 = t10590 * t2200;
    let t28903 = t791 * t3329;
    let t28904 = t8672 * t28903;
    let t28906 = F::new(0.6189328125e-1) * t28877 - F::new(0.412621875e-1) * t28880 - F::new(0.485484375e1) * t28883 + F::new(0.19419375e1) * t28885 - F::new(0.258925e1) * t28887 - F::new(0.258925e1) * t28890 - F::new(0.1294625e1) * t28892 - F::new(0.412621875e-1) * t28894 + F::new(0.16504875e0) * t28896 + F::new(0.16504875e0) * t28899 + F::new(0.82524375e-1) * t28901 - F::new(0.16504875e0) * t28904;
    (t28892, t28894, t28896, t28899, t28901, t28903, t28904, t28906)
}
