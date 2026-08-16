//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1674;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta461<F: Float>(t1828: F, t5245: F, t1277: F, t1774: F, t5497: F, t3736: F, t5428: F, t1204: F, t1210: F, t1770: F, t1775: F, t17986: F, t18054: F, t18062: F, t18087: F, t18114: F, t1829: F, t3556: F, t3561: F, t5220: F, t5246: F, t5251: F, t5414: F, t5423: F, t6580: F, t6588: F, t6697: F, t6703: F, t1811: F, t5219: F, t3737: F, t1269: F, t6628: F, t3783: F, t3769: F, t1280: F, t20703: F, t1287: F, t5284: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21366, t21382, t21390, t21393) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1674::<F>(t1828, t5245, t1277, t1774, t5497, t3736, t5428, t1204, t1210, t1770, t1775, t17986, t18054, t18062, t18087, t18114, t1829, t3556, t3561, t5220, t5246, t5251, t5414, t5423, t6580, t6588, t6697, t6703);
        let (t21394, t21408, t21416, t21427, t21430, t21436) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1675::<F>(t1811, t5219, t1828, t5497, t3737, t1269, t6628, t3783, t3769, t1280, t20703, t1287, t5284);
    (t21366, t21382, t21390, t21393, t21394, t21408, t21416, t21427, t21430, t21436)
}
