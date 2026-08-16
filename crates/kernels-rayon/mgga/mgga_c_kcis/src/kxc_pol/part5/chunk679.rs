//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 679/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk679(t1079: f64, t4621: f64, t1737: f64, t738: f64, t1742: f64, t743: f64, t1734: f64, t733: f64, t167: f64, t3153: f64, t3154: f64, t3158: f64, t3159: f64, t3161: f64, t4858: f64, t4859: f64, t4865: f64, t4866: f64, t4869: f64, t4871: f64, t4875: f64, t4881: f64) -> (f64, f64, f64, f64, f64) {
    let t4882 = t1079 * t4621;
    let t4885 = t738 * t1737;
    let t4887 = t743 * t1742;
    let t4889 = t733 * t1734;
    let t4893 = -t3153 + t3159 - 0.7026e-2_f64 * t4858 * t4859 + 0.1585e-2_f64 * t4865 * t4866 - 0.11955719325063177623e-1_f64 * t4869 + 0.10359077815592613752e-3_f64 * t4871 + 0.23911438650126355246e-1_f64 * t3158 * t167 - 0.10359077815592613752e-3_f64 * t4875 * t167 + 0.10082625e-4_f64 * t4881 * t4882 - 0.13208333333333333333e-2_f64 * t4885 - 0.117630625e-4_f64 * t4887 + 0.4684e-2_f64 * t4889 - 0.11955719325063177623e-1_f64 * t3154 + 0.10359077815592613752e-3_f64 * t3161;
    (t4882, t4885, t4887, t4889, t4893)
}
