//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1007/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1007<F: Float>(t23304: F, t5203: F, t1800: F, t1869: F, t1636: F, t8946: F, t10426: F, t5182: F, t6703: F, t6974: F, t4581: F, t8958: F, t5054: F, t1333: F, t8667: F, t7262: F, t7268: F) -> (F, F, F, F, F, F, F) {
    let t23305 = t5203 * t23304;
    let t23306 = t1800 * t23305;
    let t23307 = t1869 * t23306;
    let t23309 = t8946 * t1636;
    let t23310 = t10426 * t23309;
    let t23311 = t5182 * t23310;
    let t23313 = t6974 * t6703;
    let t23314 = t1869 * t23313;
    let t23317 = t4581 * t8958;
    let t23318 = t5054 * t23317;
    let t23320 = t1333 * t8667;
    let t23322 = t7262 * t7268;
    (t23307, t23309, t23311, t23314, t23318, t23320, t23322)
}
