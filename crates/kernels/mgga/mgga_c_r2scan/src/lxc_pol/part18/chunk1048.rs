//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1048/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1048<F: Float>(t10781: F, t9258: F, t3295: F, t9536: F, t3308: F, t6362: F, t9543: F, t11808: F, t39375: F, t8849: F, t8853: F, t11670: F, t8844: F, t1577: F, t9529: F, t9254: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43144 = t10781 * t9258;
    let t43146 = t3295 * t9536;
    let t43149 = t6362 * t3308 * t9543;
    let t43151 = t39375 * t11808;
    let t43153 = t10781 * t8849;
    let t43155 = t3295 * t8853;
    let t43157 = t11670 * t8844;
    let t43160 = t1577 * t3308 * t9529;
    let t43162 = t10781 * t9254;
    (t43144, t43146, t43149, t43151, t43153, t43155, t43157, t43160, t43162)
}
