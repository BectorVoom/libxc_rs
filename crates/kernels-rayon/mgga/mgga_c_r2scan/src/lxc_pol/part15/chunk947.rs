//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 947/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk947(t10856: f64, t2158: f64, t10813: f64, t10815: f64, t10819: f64, t10821: f64, t10824: f64, t10827: f64, t10829: f64, t10835: f64, t10837: f64, t10840: f64, t10843: f64, t10847: f64, t10851: f64, t10854: f64) -> (f64, f64) {
    let t10857 = t10856 * t2158;
    let t10859 = t10813 - 0.43341108700271342816e-1_f64 * t10815 - t10819 - 0.43663693315433241792e-2_f64 * t10821 + 0.21831846657716620896e-2_f64 * t10824 - 0.13099107994629972538e-1_f64 * t10827 + 0.43663693315433241792e-2_f64 * t10829 + t10835 + 0.21831846657716620896e-2_f64 * t10837 - t10840 + t10843 - t10847 - t10851 - t10854 - 0.97574405393827830186e-2_f64 * t10857;
    (t10857, t10859)
}
