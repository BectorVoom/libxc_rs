//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1674;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta461(t1828: f64, t5245: f64, t1277: f64, t1774: f64, t5497: f64, t3736: f64, t5428: f64, t1204: f64, t1210: f64, t1770: f64, t1775: f64, t17986: f64, t18054: f64, t18062: f64, t18087: f64, t18114: f64, t1829: f64, t3556: f64, t3561: f64, t5220: f64, t5246: f64, t5251: f64, t5414: f64, t5423: f64, t6580: f64, t6588: f64, t6697: f64, t6703: f64, t1811: f64, t5219: f64, t3737: f64, t1269: f64, t6628: f64, t3783: f64, t3769: f64, t1280: f64, t20703: f64, t1287: f64, t5284: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21366, t21382, t21390, t21393) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1674(t1828, t5245, t1277, t1774, t5497, t3736, t5428, t1204, t1210, t1770, t1775, t17986, t18054, t18062, t18087, t18114, t1829, t3556, t3561, t5220, t5246, t5251, t5414, t5423, t6580, t6588, t6697, t6703);
        let (t21394, t21408, t21416, t21427, t21430, t21436) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1675(t1811, t5219, t1828, t5497, t3737, t1269, t6628, t3783, t3769, t1280, t20703, t1287, t5284);
    (t21366, t21382, t21390, t21393, t21394, t21408, t21416, t21427, t21430, t21436)
}
