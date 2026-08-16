//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta846 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3061;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta846<F: Float>(t11424: F, t18680: F, t14913: F, t1671: F, t3264: F, t18683: F, t44162: F, t11190: F, t3307: F, t6024: F, t18265: F, t3265: F, t43969: F, t18255: F, t1117: F, t18835: F, t6021: F, t18258: F, t11185: F, t18259: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t63563, t63566, t63568, t63571, t63574) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3061::<F>(t11424, t18680, t14913, t1671, t3264, t18683, t44162, t11190, t3307, t6024, t18265, t3265, t43969);
        let (t63576, t63579, t63582, t63585, t63587) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3062::<F>(t11424, t18255, t1117, t18835, t3264, t3307, t6021, t11190, t18258, t3265, t11185, t18259);
    (t63563, t63566, t63568, t63571, t63574, t63576, t63579, t63582, t63585, t63587)
}
