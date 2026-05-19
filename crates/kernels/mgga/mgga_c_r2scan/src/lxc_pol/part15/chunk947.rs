//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 947/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk947<F: Float>(t10856: F, t2158: F, t10813: F, t10815: F, t10819: F, t10821: F, t10824: F, t10827: F, t10829: F, t10835: F, t10837: F, t10840: F, t10843: F, t10847: F, t10851: F, t10854: F) -> (F, F) {
    let t10857 = t10856 * t2158;
    let t10859 = t10813 - F::cast_from(0.43341108700271342816e-1_f64) * t10815 - t10819 - F::cast_from(0.43663693315433241792e-2_f64) * t10821 + F::cast_from(0.21831846657716620896e-2_f64) * t10824 - F::cast_from(0.13099107994629972538e-1_f64) * t10827 + F::cast_from(0.43663693315433241792e-2_f64) * t10829 + t10835 + F::cast_from(0.21831846657716620896e-2_f64) * t10837 - t10840 + t10843 - t10847 - t10851 - t10854 - F::cast_from(0.97574405393827830186e-2_f64) * t10857;
    (t10857, t10859)
}
