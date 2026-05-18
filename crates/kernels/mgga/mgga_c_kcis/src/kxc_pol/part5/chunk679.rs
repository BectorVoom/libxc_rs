//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 679/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk679<F: Float>(t1079: F, t4621: F, t1737: F, t738: F, t1742: F, t743: F, t1734: F, t733: F, t167: F, t3153: F, t3154: F, t3158: F, t3159: F, t3161: F, t4858: F, t4859: F, t4865: F, t4866: F, t4869: F, t4871: F, t4875: F, t4881: F) -> (F, F, F, F, F) {
    let t4882 = t1079 * t4621;
    let t4885 = t738 * t1737;
    let t4887 = t743 * t1742;
    let t4889 = t733 * t1734;
    let t4893 = -t3153 + t3159 - F::new(0.7026e-2) * t4858 * t4859 + F::new(0.1585e-2) * t4865 * t4866 - F::new(0.11955719325063177623e-1) * t4869 + F::new(0.10359077815592613752e-3) * t4871 + F::new(0.23911438650126355246e-1) * t3158 * t167 - F::new(0.10359077815592613752e-3) * t4875 * t167 + F::new(0.10082625e-4) * t4881 * t4882 - F::new(0.13208333333333333333e-2) * t4885 - F::new(0.117630625e-4) * t4887 + F::new(0.4684e-2) * t4889 - F::new(0.11955719325063177623e-1) * t3154 + F::new(0.10359077815592613752e-3) * t3161;
    (t4882, t4885, t4887, t4889, t4893)
}
