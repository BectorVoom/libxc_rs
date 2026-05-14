//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 846/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk846<F: Float>(t1936: F, t29432: F, t7002: F, t7586: F, t1937: F, t27060: F, t6993: F, t7316: F, t8764: F, t7239: F, t2163: F, t651: F, t7003: F, t2322: F, t8749: F, t4254: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32830 = t29432 * t1936;
    let t32832 = t7586 * t7002;
    let t32840 = t27060 * t1937;
    let t32843 = t29432 * t1937;
    let t32845 = t7586 * t6993;
    let t32849 = t8764 * t7316;
    let t32850 = t8764 * t7239;
    let t32855 = t2163 * t7002;
    let t32856 = t651 * t32855;
    let t32858 = t7586 * t7003;
    let t32862 = t2322 * t8749;
    let t32864 = t4254 * t8749;
    (t32830, t32832, t32840, t32843, t32845, t32849, t32850, t32855, t32856, t32858, t32862, t32864)
}
