//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 908/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk908<F: Float>(t10981: F, t2103: F, t3470: F, t8478: F, t8638: F, t3025: F, t9972: F, t8634: F, t955: F, t8556: F, t10010: F, t10015: F, t3447: F, t4673: F, t4752: F, t948: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10983 = 0.71500979903700853338e0 * t2103 * t10981;
    let t10988 = 0.10725146985555128001e1 * t8478 * t3470;
    let t10990 = 0.10725146985555128001e1 * t8638 * t3470;
    let t10993 = 0.10725146985555128001e1 * t3025 * t9972;
    let t10995 = 0.35750489951850426669e0 * t955 * t8634;
    let t11010 = 0.23833659967900284446e0 * t955 * t8556;
    let t11011 = 0.31952438294933958064e-1 * t10010;
    let t11012 = 0.31952438294933958064e-1 * t10015;
    let t11013 = t4673 * t3447;
    let t11015 = 0.47667319935800568892e0 * t2103 * t11013;
    let t11016 = t4752 * t948;
    (t10983, t10988, t10990, t10993, t10995, t11010, t11011, t11012, t11013, t11015, t11016)
}
