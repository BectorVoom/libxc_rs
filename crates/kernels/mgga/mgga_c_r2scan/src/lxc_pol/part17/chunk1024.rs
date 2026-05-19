//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1024/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1024<F: Float>(t1018: F, t3740: F, t1125: F, t2951: F, t2953: F, t1127: F, t2958: F, t1020: F, t1129: F, t1131: F, t1133: F, t1135: F, t2956: F, t3749: F, t3753: F, t3757: F, t3761: F) -> (F, F, F, F) {
    let t12851 = t3740 * t1018;
    let t12854 = t1125 * t2951;
    let t12856 = t1125 * t2953;
    let t12867 = t2958 * t1127;
    let t12883 = F::cast_from(0.734774460522e2_f64) * t1129 * t2958 - F::cast_from(0.11494261417236e3_f64) * t1131 * t2958 + F::cast_from(0.6202613620464e2_f64) * t1133 * t2958 - F::cast_from(0.1088826475632e2_f64) * t1135 * t2958 - F::cast_from(0.9214113627294e1_f64) * t12867 - F::cast_from(0.18428227254588e2_f64) * t3749 * t1020 - F::cast_from(0.9214113627294e1_f64) * t1129 * t2956 + F::cast_from(0.734774460522e2_f64) * t3753 * t1020 + F::cast_from(0.367387230261e2_f64) * t1131 * t2956 - F::cast_from(0.7662840944824e2_f64) * t3757 * t1020 - F::cast_from(0.3831420472412e2_f64) * t1133 * t2956 + F::cast_from(0.3101306810232e2_f64) * t3761 * t1020;
    (t12851, t12854, t12856, t12883)
}
