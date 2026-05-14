//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 892/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk892<F: Float>(t31959: F, t33803: F, t1089: F, t1668: F, t31935: F, t1976: F, t7810: F, t31892: F, t1646: F, t373: F, t372: F, t371: F, t33800: F, t8521: F, t1695: F, t1674: F, t31993: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33804 = t31959 * t33803;
    let t33808 = t31935 * t1668 * t1089;
    let t33811 = t1976 * t7810;
    let t33812 = t31892 * t33811;
    let t33815 = t373 * t1646;
    let t33816 = t372 * t33815;
    let t33817 = t371 * t33816;
    let t33822 = t33800 * t8521;
    let t33825 = t373 * t1695;
    let t33826 = t372 * t33825;
    let t33827 = t371 * t33826;
    let t33832 = t31993 * t1674;
    (t33804, t33808, t33811, t33812, t33815, t33817, t33822, t33825, t33827, t33832)
}
