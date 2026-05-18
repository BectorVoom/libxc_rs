//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1025/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1025<F: Float>(t1349: F, t1391: F, t14056: F, t14059: F, t14062: F, t14085: F, t14093: F, t158: F, t20752: F, t20754: F, t20763: F, t30153: F, t30158: F, t30838: F, t30852: F, t3819: F) -> F {
    let t30855 = -F::new(0.7026e-2) * t158 * t30838 + F::new(0.11955719325063177623e-1) * t1349 * t30158 - F::new(0.5179538907796306876e-4) * t1391 * t30158 + F::new(0.71734315950379065738e-1) * t14093 * t30153 - F::new(0.62154466893555682512e-3) * t14085 * t30153 + t14056 + t14059 - t14062 + F::new(0.10566666666666666666e-1) * t20752 + F::new(0.117630625e-3) * t20754 - F::new(0.32788e-1) * t20763 - F::new(0.71734315950379065738e-1) * t3819 * t30852;
    t30855
}
