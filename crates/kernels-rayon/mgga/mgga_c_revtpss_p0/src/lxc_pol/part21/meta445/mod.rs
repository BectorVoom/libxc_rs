//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1965;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1966;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta445(t136: f64, t1883: f64, t2457: f64, t10139: f64, t13926: f64, t543: f64, t4100: f64, t2782: f64, t10014: f64, t5741: f64, t13790: f64, t1398: f64, t10022: f64, t10066: f64, t10070: f64, t10074: f64, t10080: f64, t10085: f64, t10098: f64, t10102: f64, t14066: f64, t14203: f64, t14209: f64, t14218: f64, t213: f64, t546: f64, t1892: f64, t4086: f64, t786: f64, t4104: f64, t2470: f64, t5740: f64, t4101: f64, t1432: f64, t5763: f64, t1385: f64, t5710: f64, t10105: f64, t10109: f64, t10114: f64, t10117: f64, t10120: f64, t10126: f64, t10129: f64, t10137: f64, t10143: f64, t13921: f64, t1399: f64, t1437: f64, t3924: f64, t4118: f64, t5659: f64, t5767: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14219, t14220, t14221, t14224, t14225, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1965(t136, t1883, t2457, t10139, t13926, t543, t4100, t2782, t10014, t5741, t13790, t1398);
        let (t14231, t14237) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1966(t10022, t14230, t2782, t10066, t10070, t10074, t10080, t10085, t10098, t10102, t14066, t14203, t14209, t14218, t14221, t14227, t14229, t213, t546);
        let (t14238, t14239, t14242, t14255, t14266) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1967(t1892, t4086, t786, t4104, t2470, t5740, t4101, t1432, t5763, t1385, t5710, t10105, t10109, t10114, t10117, t10120, t10126, t10129, t10137, t10143, t13921, t1399, t1437, t3924, t4118, t5659, t5767, t820);
    (t14219, t14220, t14224, t14225, t14230, t14231, t14237, t14238, t14239, t14242, t14255, t14266)
}
