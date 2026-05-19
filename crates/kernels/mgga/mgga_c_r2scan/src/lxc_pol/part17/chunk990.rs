//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 990/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk990<F: Float>(t11758: F, t11762: F, t11766: F, t11772: F, t11774: F, t10804: F, t10813: F, t10840: F, t11417: F, t11422: F, t11768: F, t11425: F, t11782: F, t11785: F, t11788: F, t11791: F, t11795: F, t11798: F, t11800: F, t11803: F, t11806: F, t11809: F, t11812: F) -> (F, F) {
    let t12162 = F::cast_from(0.54878743191129263322e-2_f64) * t11758;
    let t12163 = F::cast_from(0.46574606203128791246e-1_f64) * t11762;
    let t12164 = F::cast_from(0.13972381860938637374e0_f64) * t11766;
    let t12166 = F::cast_from(0.46574606203128791246e-1_f64) * t11772;
    let t12167 = F::cast_from(0.10975748638225852664e-1_f64) * t11774;
    let t12168 = t12162 + t12163 - t12164 - F::cast_from(0.97574405393827830187e-2_f64) * t11768 - t12166 + t12167 + t10804 + t10813 - t11417 + t11422 - t10840;
    let t12180 = t11425 - F::cast_from(0.43663693315433241794e-2_f64) * t11782 + F::cast_from(0.43663693315433241794e-2_f64) * t11785 + F::cast_from(0.13099107994629972538e-1_f64) * t11788 + F::cast_from(0.43663693315433241794e-2_f64) * t11791 + F::cast_from(0.43663693315433241794e-2_f64) * t11795 - F::cast_from(0.86682217400542685632e-1_f64) * t11798 - F::cast_from(0.54878743191129263322e-1_f64) * t11800 + F::cast_from(0.86682217400542685632e-1_f64) * t11803 + F::cast_from(0.2600466522016280569e0_f64) * t11806 + F::cast_from(0.86682217400542685632e-1_f64) * t11809 + F::cast_from(0.2600466522016280569e0_f64) * t11812;
    (t12168, t12180)
}
