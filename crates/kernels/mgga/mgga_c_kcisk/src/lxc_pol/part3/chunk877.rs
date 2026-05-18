//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 877/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk877<F: Float>(t1175: F, t3583: F, t3539: F, t1163: F, t3587: F, t1364: F, t3544: F, t13125: F, t13131: F, t13135: F, t13140: F, t13144: F, t13150: F, t13155: F, t13158: F, t1421: F, t338: F) -> F {
    let t13161 = t3583 * t1175;
    let t13162 = t3539 * t13161;
    let t13165 = t1163 * t3587;
    let t13166 = t3539 * t13165;
    let t13169 = t3583 * t1364;
    let t13170 = t3544 * t13169;
    let t13173 = -F::new(4.0) * t338 * t13125 + F::new(0.1478346675e-2) * t1421 * t13131 - F::new(0.59133867e-2) * t1421 * t13135 + F::new(0.39422577999999999999e-2) * t1421 * t13140 + F::new(0.39422577999999999999e-2) * t1421 * t13144 + F::new(0.49278222499999999999e-2) * t1421 * t13150 - F::new(0.32852148333333333333e-2) * t1421 * t13155 + F::new(0.32852148333333333333e-2) * t1421 * t13158 + F::new(0.295669335e-2) * t1421 * t13162 + F::new(0.295669335e-2) * t1421 * t13166 - F::new(0.19711289e-2) * t1421 * t13170;
    t13173
}
