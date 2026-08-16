//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 516/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk516(t5: f64, t898: f64, t736: f64, t725: f64, t41: f64, t585: f64, t955: f64, t159: f64, t617: f64, t1856: f64, t1863: f64, t1866: f64, t1874: f64, t1875: f64, t1885: f64, t1888: f64, t1897: f64, t1901: f64, t1904: f64, t1910: f64, t1913: f64, t1916: f64, t2037: f64, t216: f64, t2483: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2788 = t898 * t5;
    let t2789 = t2788 * t736;
    let t2794 = t898 * t725;
    let t2795 = t41 * t2794;
    let t2798 = t955 * t585;
    let t2799 = t159 * t2798;
    let t2800 = t2799 * t617;
    let t2802 = -t1856 - 0.54217906501508699211e-2_f64 * t2789 - 0.21973736767207854065e-2_f64 * t2483 * t216 + 0.1350520664e0_f64 * t1863 - t2795 - 0.571528e-1_f64 * t1866 - t1874 + 4.0_f64 * t1875 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916 + 0.84681398666666666666e-3_f64 * t2800 - t2037;
    (t2788, t2789, t2794, t2795, t2798, t2799, t2800, t2802)
}
