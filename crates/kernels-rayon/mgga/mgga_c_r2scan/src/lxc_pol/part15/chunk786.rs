//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 786/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk786(t6262: f64, t784: f64, t162: f64, t9: f64, t2104: f64, t2293: f64, t2295: f64, t2302: f64, t269: f64, t550: f64, t6101: f64, t6804: f64, t6806: f64, t6809: f64, t6813: f64, t6818: f64, t6821: f64, t6826: f64, t6828: f64, t6831: f64, t6836: f64, t6839: f64, t6843: f64, t6845: f64, t6849: f64, t6855: f64, t864: f64, t870: f64) -> (f64, f64) {
    let t6856 = t784 * t6262;
    let t6860 = 1.0_f64 / t9 / t162;
    let t6868 = -6.0_f64 * t6804 * t864 + 6.0_f64 * t6806 * t6813 - 6.0_f64 * t6818 * t864 - 0.8535056841750543333e-1_f64 * t6821 * t2295 - 1.0_f64 * t6809 * t864 + 3.0_f64 * t6826 * t6828 + 0.42675284208752716665e-1_f64 * t6831 * t2295 - 1.0_f64 * t6836 * t864 - 0.42675284208752716665e-1_f64 * t6839 * t2295 + 0.60705996076593966083e-2_f64 * t6843 * t6845 - 0.1564760420987599611e0_f64 * t2293 * t6849 - 0.31914626549668908611e-4_f64 * t6855 * t6856 + 0.22258865228084454231e-1_f64 * t2302 * t2104 * t269 * t6860 - 0.24340717659807105061e0_f64 * t870 * t550 * t6101;
    (t6860, t6868)
}
