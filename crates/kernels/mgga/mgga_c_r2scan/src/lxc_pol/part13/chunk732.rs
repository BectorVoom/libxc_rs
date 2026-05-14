//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 732/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk732<F: Float>(t6262: F, t784: F, t162: F, t9: F, t2104: F, t2293: F, t2295: F, t2302: F, t269: F, t550: F, t6101: F, t6804: F, t6806: F, t6809: F, t6813: F, t6818: F, t6821: F, t6826: F, t6828: F, t6831: F, t6836: F, t6839: F, t6843: F, t6845: F, t6849: F, t6855: F, t864: F, t870: F) -> (F, F) {
    let t6856 = t784 * t6262;
    let t6860 = 1.0 / t9 / t162;
    let t6868 = -6.0 * t6804 * t864 + 6.0 * t6806 * t6813 - 6.0 * t6818 * t864 - 0.8535056841750543333e-1 * t6821 * t2295 - 1.0 * t6809 * t864 + 3.0 * t6826 * t6828 + 0.42675284208752716665e-1 * t6831 * t2295 - 1.0 * t6836 * t864 - 0.42675284208752716665e-1 * t6839 * t2295 + 0.60705996076593966083e-2 * t6843 * t6845 - 0.1564760420987599611e0 * t2293 * t6849 - 0.31914626549668908611e-4 * t6855 * t6856 + 0.22258865228084454231e-1 * t2302 * t2104 * t269 * t6860 - 0.24340717659807105061e0 * t870 * t550 * t6101;
    (t6860, t6868)
}
