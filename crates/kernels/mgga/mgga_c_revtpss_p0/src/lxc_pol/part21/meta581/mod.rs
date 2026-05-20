//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2290;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta581<F: Float>(t17974: F, t3575: F, t17807: F, t225: F, t494: F, t1209: F, t488: F, t1828: F, t3736: F, t3790: F, t3737: F, t1811: F, t3566: F, t3584: F, t1277: F, t1210: F, t12654: F, t1271: F, t1274: F, t17964: F, t17968: F, t17973: F, t1829: F, t3556: F, t3569: F, t3572: F, t3576: F, t3739: F, t460: F, t5216: F, t5220: F, t5225: F, t5237: F, t5246: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17975, t17979, t17986) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2290::<F>(t17974, t3575, t17807, t225, t494, t1209, t488);
        let (t17987, t17988, t17992, t17995, t17999, t18004) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2291::<F>(t1828, t3736, t3575, t3790, t3737, t1811, t3566, t3584, t1277, t1210, t12654, t1271, t1274, t17964, t17968, t17973, t17975, t17979, t17986, t1829, t3556, t3569, t3572, t3576, t3739, t460, t5216, t5220, t5225, t5237, t5246);
    (t17975, t17979, t17986, t17987, t17988, t17992, t17995, t17999, t18004)
}
