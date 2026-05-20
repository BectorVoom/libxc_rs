//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1965;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1966;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta445<F: Float>(t136: F, t1883: F, t2457: F, t10139: F, t13926: F, t543: F, t4100: F, t2782: F, t10014: F, t5741: F, t13790: F, t1398: F, t10022: F, t10066: F, t10070: F, t10074: F, t10080: F, t10085: F, t10098: F, t10102: F, t14066: F, t14203: F, t14209: F, t14218: F, t213: F, t546: F, t1892: F, t4086: F, t786: F, t4104: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t1385: F, t5710: F, t10105: F, t10109: F, t10114: F, t10117: F, t10120: F, t10126: F, t10129: F, t10137: F, t10143: F, t13921: F, t1399: F, t1437: F, t3924: F, t4118: F, t5659: F, t5767: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14219, t14220, t14221, t14224, t14225, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1965::<F>(t136, t1883, t2457, t10139, t13926, t543, t4100, t2782, t10014, t5741, t13790, t1398);
        let (t14231, t14237) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1966::<F>(t10022, t14230, t2782, t10066, t10070, t10074, t10080, t10085, t10098, t10102, t14066, t14203, t14209, t14218, t14221, t14227, t14229, t213, t546);
        let (t14238, t14239, t14242, t14255, t14266) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1967::<F>(t1892, t4086, t786, t4104, t2470, t5740, t4101, t1432, t5763, t1385, t5710, t10105, t10109, t10114, t10117, t10120, t10126, t10129, t10137, t10143, t13921, t1399, t1437, t3924, t4118, t5659, t5767, t820);
    (t14219, t14220, t14224, t14225, t14230, t14231, t14237, t14238, t14239, t14242, t14255, t14266)
}
