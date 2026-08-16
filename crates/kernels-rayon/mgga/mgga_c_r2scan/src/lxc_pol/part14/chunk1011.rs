//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1011/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1011(t11758: f64, t11762: f64, t11766: f64, t11772: f64, t11774: f64, t10804: f64, t10813: f64, t10840: f64, t11417: f64, t11422: f64, t11768: f64, t11425: f64, t11782: f64, t11785: f64, t11788: f64, t11791: f64, t11795: f64, t11798: f64, t11800: f64, t11803: f64, t11806: f64, t11809: f64, t11812: f64) -> (f64, f64) {
    let t12162 = 0.54878743191129263322e-2_f64 * t11758;
    let t12163 = 0.46574606203128791246e-1_f64 * t11762;
    let t12164 = 0.13972381860938637374e0_f64 * t11766;
    let t12166 = 0.46574606203128791246e-1_f64 * t11772;
    let t12167 = 0.10975748638225852664e-1_f64 * t11774;
    let t12168 = t12162 + t12163 - t12164 - 0.97574405393827830187e-2_f64 * t11768 - t12166 + t12167 + t10804 + t10813 - t11417 + t11422 - t10840;
    let t12180 = t11425 - 0.43663693315433241794e-2_f64 * t11782 + 0.43663693315433241794e-2_f64 * t11785 + 0.13099107994629972538e-1_f64 * t11788 + 0.43663693315433241794e-2_f64 * t11791 + 0.43663693315433241794e-2_f64 * t11795 - 0.86682217400542685632e-1_f64 * t11798 - 0.54878743191129263322e-1_f64 * t11800 + 0.86682217400542685632e-1_f64 * t11803 + 0.2600466522016280569e0_f64 * t11806 + 0.86682217400542685632e-1_f64 * t11809 + 0.2600466522016280569e0_f64 * t11812;
    (t12168, t12180)
}
